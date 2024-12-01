use std::{
    path::{Path, PathBuf},
    sync::{LazyLock, RwLock},
};

use crate::consts::PATHS;

static USER_PATHS: LazyLock<RwLock<Vec<String>>> = LazyLock::new(|| RwLock::new(Vec::new()));

pub fn resolve_command(command: &str) -> Option<PathBuf> {
    if command.contains('/') {
        let path = PathBuf::from(command);
        if path.exists() && path.is_file() {
            return Some(path);
        }
    } else {
        for &predefined_path in PATHS.iter() {
            let full_path = Path::new(predefined_path).join(command);
            if full_path.exists() && full_path.is_file() {
                return Some(full_path);
            }
        }

        let user_paths = USER_PATHS.read().unwrap();
        for user_path in user_paths.iter() {
            let full_path = Path::new(user_path).join(command);
            if full_path.exists() && full_path.is_file() {
                return Some(full_path);
            }
        }
    }
    None
}

pub fn add_path(new_path: &str) {
    let mut paths = USER_PATHS.write().unwrap();

    if !paths.contains(&new_path.to_string()) {
        paths.push(new_path.to_string());
    }
}

pub fn show_paths() {
    let paths = USER_PATHS.read().unwrap();

    for path in paths.iter() {
        println!("{}", path);
    }
}
