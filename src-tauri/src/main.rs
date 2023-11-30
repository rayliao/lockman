// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod config;
mod pwd;
mod utils;
use config::PRE_PWD;
use pwd::PASSWORD_MANAGER_INSTANCE;
use std::fs;
use std::io::Write;
use std::process::{Command, Stdio};
use utils::check_file;

fn init() {
    println!("执行初始化函数");
    utils::check_directory();
}

fn main() {
    init();
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            greet,
            create_volume,
            list_volume,
            load_volume
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}

fn hande_pwd(pwd: String) -> String {
    return format!(
        "{}{}{}",
        PRE_PWD,
        pwd,
        pwd[..].chars().rev().collect::<String>()
    );
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
struct CreateObject {
    name: String,
    password: String,
    size: String,
}

#[derive(serde::Serialize)]
struct ResultObject {
    msg: String,
    code: u32,
}

/**
 * 创建加密映像
 */
#[tauri::command]
fn create_volume(obj: CreateObject) -> ResultObject {
    let image_name = obj.name;
    let file_name = format!("{}.dmg", image_name);
    let savepath = utils::get_directory();
    let password = obj.password;
    let file_exist = check_file(file_name);
    if file_exist {
        ResultObject {
            msg: "文件名已经存在".to_string(),
            code: 0,
        }
    } else {
        let dmg_file_path = format!("{}{}.dmg", savepath, image_name);
        let mut child = Command::new("hdiutil")
            .arg("create")
            .arg("-encryption")
            .arg("-stdinpass")
            .arg("-size")
            .arg(obj.size)
            .arg("-volname")
            .arg(image_name)
            .arg(&dmg_file_path)
            .stdin(Stdio::piped())
            .spawn()
            .expect("Failed to execute command");
        let stdin = child.stdin.as_mut().expect("Failed to open stdin");
        writeln!(stdin, "{}", hande_pwd(password)).expect("Failed to write password");
        let output = child.wait().expect("Failed to wait for command excutetion");
        if output.success() {
            ResultObject {
                msg: "创建成功".to_string(),
                code: 1,
            }
        } else {
            ResultObject {
                msg: "创建失败".to_string(),
                code: 0,
            }
        }
    }
}

#[tauri::command]
fn list_volume() -> Vec<String> {
    let directory = utils::get_directory();
    let mut file_names: Vec<String> = Vec::new();
    if let Ok(entries) = fs::read_dir(directory) {
        for entry in entries {
            if let Ok(entry) = entry {
                let file_path = entry.path();
                if file_path.is_file() && file_path.extension() == Some("dmg".as_ref()) {
                    if let Some(file_name) = file_path.file_name() {
                        if let Some(name) = file_name.to_str() {
                            file_names.push(name.to_string());
                        }
                    }
                }
            }
        }
    }
    file_names
}

/**
 * 挂载加密映像
 */
#[tauri::command]
fn load_volume() {
    const MAX_COUNT: u32 = 2;
    let count = PASSWORD_MANAGER_INSTANCE.get_wrong_password_count();
    let image_name = "example";
    let password = "456";
    let file_name = format!("{}.dmg", image_name);
    let file_exist = check_file(file_name);
    if file_exist {
        let savepath = utils::get_directory();
        let dmg_file_path = format!("{}{}.dmg", savepath, image_name);
        let mut child = Command::new("hdiutil")
            .arg("attach")
            .arg("-stdinpass")
            .arg(&dmg_file_path)
            .stderr(Stdio::piped())
            .stdin(Stdio::piped())
            .spawn()
            .expect("");
        // let stdin = child.stdin.as_mut().expect("Failed to open stdin");
        // writeln!(stdin, "{}", hande_pwd(password)).expect("Failed to write password");
        if let Some(ref mut stdin) = child.stdin {
            stdin
                .write_all(hande_pwd(password.to_string()).as_bytes())
                .expect("Failed to write to stdin");
            // writeln!(stdin, "{}", hande_pwd(password)).expect("");
        }
        let output = child
            .wait_with_output()
            .expect("Failed to wait for command excutetion");
        if !output.status.success() {
            // 检查输出是否包含特定的错误信息
            let stderr = String::from_utf8_lossy(&output.stderr);
            println!("{}", &stderr);
            if stderr.contains("attach failed") {
                if count >= MAX_COUNT {
                    println!("输错密码三次了！！！");
                    // TODO：直接删掉映像
                } else {
                    println!("密码错误{}", count);
                    PASSWORD_MANAGER_INSTANCE.process_password(false);
                }
            }
        } else {
            PASSWORD_MANAGER_INSTANCE.process_password(true);
        }
    } else {
        // TODO
    }
}
