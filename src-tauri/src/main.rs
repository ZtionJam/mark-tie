// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::thread;

use crate::action::*;
use crate::proxy::start_proxy_server;

mod tie;
mod constants;
mod action;
mod util;
mod config;
mod proxy;


fn main() {
    thread::spawn(|| {
        start_proxy_server()
    });

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_index_page,get_topic,get_hot_forum,get_or_set_cookie,get_user_info,get_config,get_feed_info])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
