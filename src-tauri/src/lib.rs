extern crate core;

mod app_state;
pub mod countdown;

use crate::countdown::commands::{
    countdown_create, countdown_delete, countdown_list, countdown_pause, countdown_reset,
    countdown_resume, countdown_snapshot, countdown_start,
};
pub use app_state::AppState;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let app_state = AppState::new();
    tauri::Builder::default()
        .manage(app_state)
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            countdown_create,
            countdown_list,
            countdown_delete,
            countdown_start,
            countdown_reset,
            countdown_resume,
            countdown_pause,
            countdown_snapshot,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
