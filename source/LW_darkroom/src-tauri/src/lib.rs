
pub mod mediapool;
pub mod appstate;
pub mod rawengine;
use crate::appstate::AppState;
use crate::mediapool::scanner::get_assets;
use std::sync::Mutex;


// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

use crate::mediapool::scanner::scan_and_build;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .manage(Mutex::new(AppState::new()))
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![greet, scan_and_build, get_assets])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
