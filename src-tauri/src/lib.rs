extern crate core;

mod app_state;
pub mod countdown;
mod server;

use std::sync::Arc;

use crate::app_state::AppState;
use crate::countdown::commands::{
    countdown_create, countdown_delete, countdown_list, countdown_pause, countdown_reset,
    countdown_resume, countdown_snapshot, countdown_start, set_overlay_config, spawn_ticker,
};
use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let app_state = Arc::new(AppState::new());

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
            set_overlay_config,
        ])
        .build(tauri::generate_context!())
        .expect("error while building tauri application")
        .run(|app_handle, event| {
            if let tauri::RunEvent::Ready = event {
                let axum_state = app_handle.state::<Arc<AppState>>().inner().clone();
                tauri::async_runtime::spawn(async move { server::start(axum_state).await; });
                spawn_ticker(app_handle.clone());
            }
        });
}
