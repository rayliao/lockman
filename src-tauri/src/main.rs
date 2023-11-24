// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod utils;
use std::{
    env,
    process::{exit, Command},
};

fn init() {
    println!("执行初始化函数");
    utils::check_directory();
}

fn main() {
    init();
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
        .invoke_handler(tauri::generate_handler![create_volume])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}

/**
 * 创建加密映像
 */
#[tauri::command]
fn create_volume() {
    let image_name = "example";
    let file_name = format!("{}.dmg", image_name);
    let image_size = 1024;
    let savepath = utils::get_savepath();
    let password = "123456";
    let mut command = Command::new("hdiutil");
    if utils::check_file(file_name) {
        println!("文件名已经存在");
    } else {
        command.arg("create");
        command.arg("-size").arg(format!("{}", image_size));
        // command.arg("-fs").arg("HFS+");
        command.arg("-volname").arg(image_name);
        let dmg_file_path = format!("{}{}.dmg", savepath, image_name);
        command.arg(&dmg_file_path);
        let pwd = format!(
            "lm{}{}",
            password,
            password[..].chars().rev().collect::<String>()
        );
        command.arg("-password").arg(pwd);
        match command.status() {
            Ok(status) => {
                if status.success() {
                    println!("成功创建空白映像！");
                } else {
                    println!("命令执行失败:{:?}", status.code());
                }
            }
            Err(e) => {
                eprintln!("无法启动进程:{}", e);
                exit(1);
            }
        }
    }
}
