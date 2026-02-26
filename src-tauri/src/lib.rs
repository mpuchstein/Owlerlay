extern crate core;

mod app_state;
mod countdown;

use crate::countdown::commands::{
    countdown_pause, countdown_reset, countdown_resume, countdown_snapshot, countdown_start,
};
pub use app_state::AppState;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let app_state = AppState::new();
    tauri::Builder::default()
        .manage(app_state)
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            greet,
            countdown_start,
            countdown_reset,
            countdown_pause,
            countdown_resume,
            countdown_snapshot,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
