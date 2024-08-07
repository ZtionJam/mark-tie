use std::sync::{Arc, Mutex};

use serde_json::Value;
use tauri::State;

use crate::constants::{client, url};
use crate::domain::dto::CookieCheck;
use crate::domain::ex::AppConfig;
use crate::util::http::get_cookie_header;

///检查cookie有效性
#[tauri::command]
pub fn check_cookie(
    cc: CookieCheck,
    app_config: State<'_, Arc<Mutex<AppConfig>>>,
) -> Result<(), String> {
    if cc.save {
        let mut config = app_config.lock().unwrap();
        config.cookie = cc.cookie.clone();
        config.save();
    }

    if "success".eq(&request_sync(&cc.cookie)) {
        return Ok(());
    }

    Err("Login failed".to_string())
}
///检查配置中的cookie有效性
#[tauri::command]
pub fn check_login(app_config: State<'_, Arc<Mutex<AppConfig>>>) -> Result<(), String> {
    let config = app_config.lock().unwrap();
    let err_code = request_sync(&config.cookie);
    if "success".eq(&err_code) {
        return Ok(());
    }
    Err(err_code)
}

pub fn request_sync(cookie: &String) -> String {
    let body = &client::CLIENT
        .get(url::SYNC)
        .headers(get_cookie_header(cookie))
        .send()
        .unwrap()
        .text()
        .unwrap();
    let ret: Value = serde_json::from_str(body).unwrap();
    ret.get("error")
        .and_then(|v| v.as_str().map(String::from))
        .unwrap_or_else(|| "Cookie not valid".to_string())
}
