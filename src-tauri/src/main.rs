// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{thread, os::windows};

use crate::action::*;
use crate::proxy::start_proxy_server;

mod action;
mod config;
mod constants;
mod proxy;
mod tie;
mod util;

use tauri::Manager;
use windows::{
    core::Result,
    Win32::{
        Foundation::{HWND, LPARAM, WPARAM},
        UI::WindowsAndMessaging::{GetWindowLongPtrW, SetWindowLongPtrW, GWL_EXSTYLE, WS_EX_LAYERED},
    },
};
fn add_shadow(hwnd: HWND) -> Result<()> {
    unsafe {
        let current_style = GetWindowLongPtrW(hwnd, GWL_EXSTYLE);
        SetWindowLongPtrW(
            hwnd,
            GWL_EXSTYLE,
            current_style | WS_EX_LAYERED as isize,
        );
    }
    Ok(())
}
fn main() {
    thread::spawn(|| start_proxy_server());

    tauri::Builder::default()
        .setup(|app| {
            let main_window = app.get_window("main").unwrap();
            let hwnd = HWND(main_window.hwnd().unwrap() as u32); // 正确的转换
            add_shadow(hwnd)?;

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
