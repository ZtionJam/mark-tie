use std::sync::{Arc, Mutex};

use reqwest::Body;
use tauri::State;

use crate::constants::{client, header,url};
use crate::domain::dto::CookieCheck;
use crate::domain::ex::AppConfig;
use crate::util::http::{get_cookie_header, get_now_header};

///检查cookie有效性
#[tauri::command]
pub fn check_cookie(cc: CookieCheck, state: State<'_, Arc<Mutex<AppConfig>>>) {
    if cc.save {
        state.lock().unwrap().cookie = cc.cookie;
    }
    let body = &client::CLIENT
        .get(url::SYNC)
        .headers(get_now_header())
        .send()
        .unwrap()
        .text()
        .unwrap();
}
