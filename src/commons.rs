use crate::SLASH;
use std::path::Path;

pub fn greeter() {
    println!("welcome to ferrum");
    println!("type 'exit' to leave (optional)");
}

pub fn trim_path(path: &Path) -> String {
    let home = dirs::home_dir().unwrap_or_else(|| Path::new(SLASH).to_path_buf());
    if let Ok(stripped) = path.strip_prefix(&home) {
        let parts: Vec<&str> = stripped
            .components()
            .map(|component| component.as_os_str().to_str().unwrap_or(""))
            .collect();
        let trimmed_parts = parts
            .iter()
            .enumerate()
            .map(|(i, part)| {
                if i == 0 || i == parts.len() - 1 {
                    part.to_string()
                } else {
                    part.chars()
                        .next()
                        .map_or("".to_string(), |c| c.to_string())
                }
            })
            .collect::<Vec<String>>()
            .join("/");
        format!(
            "~/{}",
            if trimmed_parts.is_empty() {
                ""
            } else {
                &trimmed_parts
            }
        )
    } else {
        path.display().to_string()
    }
}
