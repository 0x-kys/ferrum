use std::{
    path::{Path, PathBuf},
    sync::{LazyLock, RwLock},
};

pub const PATHS: [&'static str; 3] = ["/bin", "/usr/bin", "/usr/local/bin"];

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

pub fn add_path(new_paths: &Vec<String>) {
    let mut paths = USER_PATHS.write().unwrap();

    for path in new_paths {
        if !paths.contains(&path) {
            paths.push(path.to_string());
        }
    }
}

pub fn predefined_paths() -> Vec<String> {
    PATHS.iter().map(|&s| s.to_string()).collect()
}

pub fn show_paths() {
    let paths = USER_PATHS.read().unwrap();
    for path in paths.iter() {
        println!("{}", path);
    }
}
