use once_cell::sync::Lazy;
use std::sync::Mutex;

pub struct PasswordManager {
    pub wrong_password_count: Mutex<u32>,
}

// 使用 once_cell 实现单例
pub static PASSWORD_MANAGER_INSTANCE: Lazy<PasswordManager> = Lazy::new(|| PasswordManager::new());

impl PasswordManager {
    pub fn new() -> PasswordManager {
        PasswordManager {
            wrong_password_count: Mutex::new(0),
        }
    }
    pub fn process_password(&self, reset: bool) {
        let mut count = self.wrong_password_count.lock().unwrap();
        if reset {
            *count = 0;
            println!("Password is correct!");
        } else {
            *count += 1;
            println!("Incorrect password! Attempts: {}", *count);
        }
    }
    pub fn get_wrong_password_count(&self) -> u32 {
        match self.wrong_password_count.lock() {
            Ok(guard) => *guard,
            Err(_) => {
                println!("Failed to lock the mutex.");
                0
            }
        }
    }
}
