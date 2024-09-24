use std::fs;
use std::path::Path;
use std::process::exit;

pub fn get_directory(file: &str) -> String {
    let mut path = file.to_string().replace("\\", "/");
    if path.contains("/") {
        let split_parts = path.split("/").collect::<Vec<_>>();
        path = split_parts[0..split_parts.len() - 1].join("/");
    } else {
        path = ".".to_string();
    }
    path
}

pub fn create_directory(path: &str) {
    if !Path::new(path).exists() && fs::create_dir_all(path).is_err() {
        show_error("Failed to create directory")
    }
}

pub fn show_error(msg: &str) -> ! {
    eprintln!("\x1b[1;31m{}\x1b[0m", msg);
    exit(1)
}
