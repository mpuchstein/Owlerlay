//! Persistent overlay storage.
//!
//! Groups *and* per-countdown styling live together in `<app_local_data_dir>/overlays.json`.
//! Co-located because they're both overlay-module data, always small together,
//! and the save path can flip both atomically — never risk a partial update
//! where, say, a group rename persists but its members' configs don't. (Same
//! single-store-per-widget rule AGENTS.md calls out for the countdown store.)
//!
//! `next_id` isn't stored: like the countdown store, it's derived as
//! `max(group.id)+1` on restore (a deleted group doesn't matter — we only
//! need to avoid colliding with a restored id).

use std::collections::HashMap;

use tauri::AppHandle;

use crate::overlay::dto::{GroupDto, GroupsAndConfigsDto};
use crate::overlay::model::OverlayConfig;

fn store_path(handle: &AppHandle) -> std::io::Result<std::path::PathBuf> {
    crate::settings::local_data_file(handle, "overlays.json")
}

/// Load persisted overlay state, falling back to empty so a corrupt store
/// never bricks startup. The on-disk shape is the full
/// [`GroupsAndConfigsDto`] (serialized as a JSON object with a `groups`
/// array and a `configs` map) — there is no separate "format" to maintain
/// aside from the DTO itself.
///
/// A *missing* file is the normal first-run case; a file that's present but
/// unparseable is moved aside to `overlays.json.corrupt` before returning
/// empty, mirroring the countdown store — a corrupt save never silently
/// overwrites the user's groups and styling, and they stay recoverable by hand.
pub fn load(handle: &AppHandle) -> (Vec<GroupDto>, HashMap<u64, OverlayConfig>) {
    let Ok(path) = store_path(handle) else {
        return (Vec::new(), HashMap::new());
    };
    let Ok(contents) = std::fs::read_to_string(&path) else {
        return (Vec::new(), HashMap::new());
    };
    match serde_json::from_str::<GroupsAndConfigsDto>(&contents) {
        Ok(parsed) => parsed.into_parts(),
        Err(_) => {
            // ponytail: single .corrupt slot; a later corruption overwrites it,
            // matching the countdown store.
            let _ = std::fs::rename(&path, path.with_extension("json.corrupt"));
            (Vec::new(), HashMap::new())
        }
    }
}

/// Persist the current overlay state. Fire-and-forget: the (tiny) serialize
/// runs on the caller, then the blocking disk write is handed to a
/// blocking thread (`spawn_blocking`) so it never stalls the async runtime
/// that drives the 100ms countdown ticker or the LAN-remote SSE stream.
/// Errors are dropped — every call site is best-effort, and a missed save
/// just means the next mutation will rewrite the same shape anyway.
pub fn save(handle: &AppHandle, groups: &[GroupDto], configs: &HashMap<u64, OverlayConfig>) {
    let Ok(path) = store_path(handle) else {
        return;
    };
    let dto = GroupsAndConfigsDto::from_parts(groups.iter().cloned(), configs);
    let Ok(json) = serde_json::to_string_pretty(&dto) else {
        return;
    };
    tauri::async_runtime::spawn_blocking(move || {
        let _ = crate::settings::write_atomic(&path, &json, None);
    });
}
