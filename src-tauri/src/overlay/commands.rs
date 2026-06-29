use crate::app_state::AppState;
use crate::countdown::events::AppEvent;
use crate::overlay::dto::GroupDto;
use crate::overlay::model::{Layout, OverlayConfig};
use std::sync::Arc;
use tauri::{AppHandle, State, command};

/// Save the current overlay state to disk. Centralised here so every
/// mutating command has exactly one place to emit the persist — the same
/// fire-and-forget pattern as `countdown::store::save` uses for its commands.
/// Called *after* the in-memory mutation so a failed save can never diverge
/// from the live state.
async fn save_overlays(app: &AppHandle, state: &AppState) {
    let (groups, configs) = state.overlay_service.snapshot().await;
    crate::overlay::store::save(app, &groups, &configs);
}

#[command]
pub async fn group_create(
    app: AppHandle,
    state: State<'_, Arc<AppState>>,
    name: String,
) -> Result<u64, String> {
    let id = state
        .overlay_service
        .create_group(name)
        .await
        .map_err(|e| e.to_string())?;
    save_overlays(&app, &state).await;
    Ok(id)
}

#[command]
pub async fn group_list(state: State<'_, Arc<AppState>>) -> Result<Vec<GroupDto>, String> {
    Ok(state
        .overlay_service
        .list_groups()
        .await
        .into_iter()
        .map(GroupDto::from)
        .collect())
}

#[command]
pub async fn group_update(
    app: AppHandle,
    state: State<'_, Arc<AppState>>,
    id: u64,
    name: String,
    members: Vec<u64>,
    layout: Layout,
    hide_idle: bool,
) -> Result<(), String> {
    state
        .overlay_service
        .update_group(id, name, members, layout, hide_idle)
        .await
        .map_err(|e| e.to_string())?;
    save_overlays(&app, &state).await;
    let _ = state.event_bus.send(AppEvent::Reload);
    Ok(())
}

#[command]
pub async fn group_delete(
    app: AppHandle,
    state: State<'_, Arc<AppState>>,
    id: u64,
) -> Result<(), String> {
    state
        .overlay_service
        .delete_group(id)
        .await
        .map_err(|e| e.to_string())?;
    save_overlays(&app, &state).await;
    let _ = state.event_bus.send(AppEvent::Reload);
    Ok(())
}

#[command]
pub async fn set_overlay_config(
    app: AppHandle,
    state: State<'_, Arc<AppState>>,
    id: u64,
    config: OverlayConfig,
) -> Result<(), String> {
    state.overlay_service.set_config(id, config).await;
    save_overlays(&app, &state).await;
    let _ = state.event_bus.send(AppEvent::Reload);
    Ok(())
}

/// Read the current per-countdown overlay config. Exposes the existing
/// [`OverlayService::get_config`] (which already returns `Default` when
/// nothing is stored) so the frontend can hydrate
/// [`AppearancePanel`](../../../../features/countdown/components/AppearancePanel.svelte)
/// on mount — fixing both the view-switch bug (the local `$state` dict
/// being blown away when the panel unmounts) and the design-lost-on-restart
/// symptom (the in-memory map being empty until the user opens the panel
/// and a save round-trip repopulates it).
#[command]
pub async fn get_overlay_config(
    state: State<'_, Arc<AppState>>,
    id: u64,
) -> Result<OverlayConfig, String> {
    Ok(state.overlay_service.get_config(id).await)
}
