// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{os::windows, thread};

use tauri::Manager;
use window_shadows::set_shadow;

use crate::action::*;
use crate::proxy::start_proxy_server;

mod action;
mod config;
mod constants;
mod proxy;
mod tie;
mod util;

fn main() {
    thread::spawn(|| start_proxy_server());

    tauri::Builder::default()
        .setup(|app| {
            let main_window = app.get_window("main").unwrap();
            #[cfg(any(windows, target_os = "macos"))]
            set_shadow(&main_window, true).unwrap();
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            get_index_page,
            get_topic,
            get_hot_forum,
            get_or_set_cookie,
            get_user_info,
            get_config,
            get_feed_info,
            get_feed_comment
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
