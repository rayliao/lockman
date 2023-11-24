use std::path::Path;
use std::{env, fs};

pub fn get_savepath() -> String {
    let home_dir = env::var("HOME").unwrap();
    let username = home_dir.split("/").last().unwrap();
    return format!("/Users/{}/{}/", username, "lockman");
}

pub fn check_directory() {
    let savepath = get_savepath();
    let path = Path::new(&savepath);
    if !path.exists() {
        fs::create_dir_all(path).unwrap();
        println!("The directory has been created.");
    } else {
        println!("The directory already exists.");
    }
}

pub fn check_file(name: String) -> bool {
    let savepath = get_savepath();
    let path = Path::new(&savepath);
    return path.join(name).exists();
}
