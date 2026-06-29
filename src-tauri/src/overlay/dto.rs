use crate::overlay::model::{Group, Layout, OverlayConfig};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, HashMap};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GroupDto {
    pub id: u64,
    pub name: String,
    pub members: Vec<u64>,
    pub layout: Layout,
    pub hide_idle: bool,
}

impl From<Group> for GroupDto {
    fn from(g: Group) -> Self {
        Self {
            id: g.id,
            name: g.name,
            members: g.members,
            layout: g.layout,
            hide_idle: g.hide_idle,
        }
    }
}

/// On-disk shape of `<app_local_data_dir>/overlays.json`.
///
/// One DTO holding both the groups and their per-countdown styling so a save
/// is always atomic across the two — the alternative (two files, two
/// writes) risks a partial commit where a group is renamed but its members'
/// configs are lost. Configs are serialized as `{"<id>": OverlayConfig}`
/// keyed by the countdown id (an `OverlayConfigMapEntry` newtype wrapper
/// keeps `serde` honest on the value type).
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GroupsAndConfigsDto {
    #[serde(default)]
    pub groups: Vec<GroupDto>,
    #[serde(default)]
    pub configs: BTreeMap<String, OverlayConfigMapEntry>,
}

/// Wrapper so the `BTreeMap<String, _>` value type serializes as the inner
/// `OverlayConfig` (and not as a `{"value": ...}` envelope). The newtype
/// is serialized transparently.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct OverlayConfigMapEntry(pub OverlayConfig);

impl GroupsAndConfigsDto {
    /// Build the DTO from in-memory state. BTreeMap is used so the on-disk
    /// JSON output is stable / diff-friendly across saves.
    pub fn from_parts<'a>(
        groups: impl IntoIterator<Item = GroupDto>,
        configs: &'a HashMap<u64, OverlayConfig>,
    ) -> Self {
        let mut tree: BTreeMap<String, OverlayConfigMapEntry> = BTreeMap::new();
        for (id, cfg) in configs {
            tree.insert(id.to_string(), OverlayConfigMapEntry(cfg.clone()));
        }
        Self {
            groups: groups.into_iter().collect(),
            configs: tree,
        }
    }

    /// Split back into in-memory shapes. Drops configs whose key doesn't
    /// parse as a `u64` (kept alongside the corrupt-quarantine so a hand-
    /// edited file can't crash a future save).
    pub fn into_parts(self) -> (Vec<GroupDto>, HashMap<u64, OverlayConfig>) {
        let mut map: HashMap<u64, OverlayConfig> =
            HashMap::with_capacity(self.configs.len());
        for (k, OverlayConfigMapEntry(cfg)) in self.configs {
            if let Ok(id) = k.parse::<u64>() {
                map.insert(id, cfg);
            }
        }
        (self.groups, map)
    }
}
