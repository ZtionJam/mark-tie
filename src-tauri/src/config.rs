use std::fs::{metadata, OpenOptions};
use std::io::{Read, Write};
use std::path::Path;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
    pub cookie: String,
    pub port: String,
}

impl Config {
    ///更新保存配置
    pub fn flush(&self) {
        let file_path = Path::new("config.json");
        let mut file = match OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(&file_path) {
            Ok(file) => { file }
            Err(e) => {
                eprintln!("Error opening file: {:?}", e);
                panic!("读取或创建配置失败，请确保程序权限")
            }
        };
        let default_config = serde_json::to_string(self).unwrap();
        if let Err(_) = file.write_all(default_config.as_bytes()) {
            panic!("创建配置失败，请确保程序权限")
        }
    }
    ///
    /// 读取运行目录下的config.json配置
    pub fn read() -> Self {
        let mut config: Config = Config::new();

        let file_path = Path::new("config.json");
        let mut file = match OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(&file_path) {
            Ok(file) => { file }
            Err(e) => {
                eprintln!("Error opening file: {:?}", e);
                panic!("读取或创建配置失败，请确保程序权限")
            }
        };

        let meta = metadata(&file_path).unwrap();

        if meta.len() == 0 {
            config.flush();
        } else {
            let mut content = String::new();
            if let Err(_) = file.read_to_string(&mut content) {
                panic!("读取配置失败，请确保程序权限")
            }
            config = match serde_json::from_str(&*content) {
                Ok(c) => { c }
                Err(e) => {
                    println!("读取配置失败，请确保程序权限,使用默认配置.{}", e);
                    config.flush();
                    config
                }
            }
        }

        config
    }

    fn new()->Self{
        Config{
            cookie:"".to_string(),
            port:String::from("63343")
        }
    }
}