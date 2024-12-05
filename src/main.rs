mod commons;
mod consts;
mod path;

use std::{
    env,
    io::{stdin, stdout, Write},
    path::Path,
    process::{Child, Command, Stdio},
};

use ansi_term::Color;
use commons::trim_path;

use crate::{
    commons::greeter,
    consts::{PIPE, PROMPT, SLASH},
    path::{add_path, resolve_command, show_paths},
};

fn main() {
    greeter();

    add_path(&path::predefined_paths());

    loop {
        let current_dir = env::current_dir().unwrap_or_else(|_| Path::new(SLASH).to_path_buf());
        let prompt = format!(
            "{} {}",
            Color::Yellow.paint(trim_path(&current_dir)),
            Color::Green.bold().paint(PROMPT)
        );

        print!("{}", prompt);
        let _ = stdout().flush();

        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        let mut commands = input.trim().split(PIPE).peekable();
        let mut previous_command = None;

        while let Some(command) = commands.next() {
            let mut parts = command.trim().split_whitespace();
            let command = parts.next().unwrap();
            let args = parts;

            match command {
                "exit" => return,
                "cd" => {
                    let new_dir = args.peekable().peek().map_or(SLASH, |x| *x);
                    let root = Path::new(new_dir);
                    if let Err(e) = env::set_current_dir(&root) {
                        eprintln!("{}", e);
                    }

                    previous_command = None;
                }
                "setpath" => {
                    let new_paths = args.map(|s| s.to_string()).collect::<Vec<String>>();

                    if new_paths.is_empty() {
                        eprintln!("Usage: setpath <path> [<path> ...]");
                    } else {
                        let path_to_add = new_paths.clone();

                        add_path(&path_to_add);

                        println!("Paths added: {:?}", path_to_add);
                    }
                }
                "showpath" => {
                    show_paths();
                }
                command => {
                    let stdin = previous_command.map_or(Stdio::inherit(), |output: Child| {
                        Stdio::from(output.stdout.unwrap())
                    });

                    let stdout = if commands.peek().is_some() {
                        Stdio::piped()
                    } else {
                        Stdio::inherit()
                    };

                    match resolve_command(command) {
                        Some(full_path) => {
                            let output = Command::new(full_path)
                                .args(args)
                                .stdin(stdin)
                                .stdout(stdout)
                                .spawn();

                            match output {
                                Ok(output) => {
                                    previous_command = Some(output);
                                }
                                Err(e) => {
                                    previous_command = None;
                                    eprintln!("Error executing '{}': {}", command, e);
                                }
                            };
                        }
                        None => {
                            eprintln!("Command not found: {}", command);
                            previous_command = None;
                        }
                    }
                }
            }
        }

        if let Some(mut final_command) = previous_command {
            let _ = final_command.wait();
        }
    }
}

