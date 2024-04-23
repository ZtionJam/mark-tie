// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use crate::action::*;

mod tie;
mod constants;
mod action;
mod util;

#[tauri::command]
async fn greet(name: &str) -> Result<String, ()> {
    Ok("".to_string())
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_index_page,get_topic,get_hot_forum])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
