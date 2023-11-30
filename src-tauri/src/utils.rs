use std::path::Path;
use std::{env, fs};

pub fn get_directory() -> String {
    let home_dir = env::var("HOME").unwrap();
    let username = home_dir.split("/").last().unwrap();
    return format!("/Users/{}/{}/", username, "lockman");
}

pub fn check_directory() {
    let directory = get_directory();
    let path = Path::new(&directory);
    if !path.exists() {
        fs::create_dir_all(path).unwrap();
        println!("The directory has been created.");
    } else {
        println!("The directory already exists.");
    }
}

pub fn check_file(name: String) -> bool {
    let directory = get_directory();
    let path = Path::new(&directory);
    return path.join(name).exists();
}
