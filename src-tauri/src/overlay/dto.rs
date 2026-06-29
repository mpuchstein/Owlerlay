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
/// configs are lost. `serde_json` writes the `u64`-keyed map as
/// `{"<id>": OverlayConfig}` natively (integer keys are stringified on write,
/// parsed back on read); the `BTreeMap` keeps that output sorted/diff-friendly.
/// A non-numeric key in a hand-edited file is a parse error, so `store::load`
/// quarantines the whole file to `.json.corrupt` rather than silently dropping
/// the entry — the same all-or-nothing rule the countdown store uses.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GroupsAndConfigsDto {
    #[serde(default)]
    pub groups: Vec<GroupDto>,
    #[serde(default)]
    pub configs: BTreeMap<u64, OverlayConfig>,
}

impl GroupsAndConfigsDto {
    /// Build the DTO from in-memory state (sorted into a `BTreeMap` so the
    /// on-disk JSON stays stable / diff-friendly across saves).
    pub fn from_parts(
        groups: impl IntoIterator<Item = GroupDto>,
        configs: &HashMap<u64, OverlayConfig>,
    ) -> Self {
        Self {
            groups: groups.into_iter().collect(),
            configs: configs.iter().map(|(id, cfg)| (*id, cfg.clone())).collect(),
        }
    }

    /// Split back into the in-memory shapes the service holds.
    pub fn into_parts(self) -> (Vec<GroupDto>, HashMap<u64, OverlayConfig>) {
        (self.groups, self.configs.into_iter().collect())
    }
}
