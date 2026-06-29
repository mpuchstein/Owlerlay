use crate::overlay::dto::GroupDto;
use crate::overlay::errors::OverlayError;
use crate::overlay::model::{Group, Layout, OverlayConfig};
use std::collections::HashMap;
use tokio::sync::Mutex;

const MAX_GROUPS: usize = 20;

/// In-memory store of overlay groups and per-countdown styling, guarded by
/// async mutexes (mirrors [`CountdownService`](crate::countdown::service::CountdownService)).
#[derive(Debug)]
pub struct OverlayService {
    groups: Mutex<HashMap<u64, Group>>,
    configs: Mutex<HashMap<u64, OverlayConfig>>,
    next_id: Mutex<u64>,
}

impl Default for OverlayService {
    fn default() -> Self {
        Self::new()
    }
}

impl OverlayService {
    pub fn new() -> Self {
        Self {
            groups: Mutex::new(HashMap::new()),
            configs: Mutex::new(HashMap::new()),
            next_id: Mutex::new(0),
        }
    }

    /// Rebuild the service from persisted [`GroupDto`]s + per-id configs.
    /// `next_id` is derived as `max(group.id)+1` over the *full* input (so it
    /// can't collide with a retained entry); a saturating add defends against a
    /// hand-edited store where `max(id) == u64::MAX` — overflow would otherwise
    /// panic on boot. Empty input starts ids at 0, matching the fresh-service
    /// default. The same corrupt-store defenses [`CountdownService::from_dtos`]
    /// applies hold here: cap restored groups at [`MAX_GROUPS`] (the limit
    /// `create_group` enforces), drop blank-named groups it would itself reject,
    /// and keep the first entry per duplicate id (no nondeterministic drop).
    pub fn from_groups_and_configs(
        groups: Vec<GroupDto>,
        configs: HashMap<u64, OverlayConfig>,
    ) -> Self {
        let next_id = groups
            .iter()
            .map(|g| g.id)
            .max()
            .map_or(0, |m| m.saturating_add(1));
        let mut groups_map: HashMap<u64, Group> =
            HashMap::with_capacity(groups.len().min(MAX_GROUPS));
        for g in groups {
            if groups_map.len() >= MAX_GROUPS {
                break;
            }
            if g.name.trim().is_empty() {
                continue;
            }
            // First entry per id wins. A duplicate id in a hand-edited store
            // can't silently drop a group in nondeterministic HashMap order.
            groups_map.entry(g.id).or_insert_with(|| Group {
                id: g.id,
                name: g.name,
                members: g.members,
                layout: g.layout,
                hide_idle: g.hide_idle,
            });
        }
        Self {
            groups: Mutex::new(groups_map),
            configs: Mutex::new(configs),
            next_id: Mutex::new(next_id),
        }
    }

    /// Snapshot the current overlay state into the persisted DTO shape.
    /// Mirrors the in-memory map back to disk-safe types so the save path
    /// doesn't reach into the service's internals.
    pub async fn snapshot(&self) -> (Vec<GroupDto>, HashMap<u64, OverlayConfig>) {
        let groups: Vec<GroupDto> = self
            .groups
            .lock()
            .await
            .values()
            .cloned()
            .map(GroupDto::from)
            .collect();
        let configs = self.configs.lock().await.clone();
        (groups, configs)
    }

    pub async fn create_group(&self, name: String) -> Result<u64, OverlayError> {
        let name = name.trim().to_string();
        if name.is_empty() {
            return Err(OverlayError::EmptyName);
        }
        let mut groups = self.groups.lock().await;
        if groups.len() >= MAX_GROUPS {
            return Err(OverlayError::MaxGroupsReached);
        }
        let mut next_id = self.next_id.lock().await;
        let id = *next_id;
        // saturating_add matches the restore path's overflow contract — the two
        // halves of the same counter must defend identically.
        *next_id = next_id.saturating_add(1);
        groups.insert(
            id,
            Group {
                id,
                name,
                members: Vec::new(),
                layout: Layout::default(),
                hide_idle: false,
            },
        );
        Ok(id)
    }

    pub async fn list_groups(&self) -> Vec<Group> {
        let mut groups: Vec<Group> = self.groups.lock().await.values().cloned().collect();
        groups.sort_by_key(|g| g.id);
        groups
    }

    pub async fn get_group(&self, id: u64) -> Option<Group> {
        self.groups.lock().await.get(&id).cloned()
    }

    pub async fn update_group(
        &self,
        id: u64,
        name: String,
        members: Vec<u64>,
        layout: Layout,
        hide_idle: bool,
    ) -> Result<(), OverlayError> {
        let name = name.trim().to_string();
        if name.is_empty() {
            return Err(OverlayError::EmptyName);
        }
        let mut groups = self.groups.lock().await;
        let group = groups.get_mut(&id).ok_or(OverlayError::GroupNotFound)?;
        group.name = name;
        group.members = members;
        group.layout = layout;
        group.hide_idle = hide_idle;
        Ok(())
    }

    pub async fn delete_group(&self, id: u64) -> Result<(), OverlayError> {
        if self.groups.lock().await.remove(&id).is_some() {
            Ok(())
        } else {
            Err(OverlayError::GroupNotFound)
        }
    }

    pub async fn set_config(&self, id: u64, config: OverlayConfig) {
        self.configs.lock().await.insert(id, config);
    }

    pub async fn get_config(&self, id: u64) -> OverlayConfig {
        self.configs
            .lock()
            .await
            .get(&id)
            .cloned()
            .unwrap_or_default()
    }

    /// Drop the persisted styling for a countdown that no longer exists.
    /// Called from the countdown delete path so a deleted timer's config can't
    /// linger in `overlays.json` forever (or resurrect onto a later reused id).
    /// Returns whether anything was removed, so the caller only re-saves when
    /// there was a config to prune.
    pub async fn remove_config(&self, id: u64) -> bool {
        self.configs.lock().await.remove(&id).is_some()
    }
}
