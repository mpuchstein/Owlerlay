use owlerlay_lib::overlay::dto::{GroupDto, GroupsAndConfigsDto, OverlayConfigMapEntry};
use owlerlay_lib::overlay::model::{Layout, OverlayConfig};
use owlerlay_lib::overlay::service::OverlayService;

#[test]
fn overlay_config_deserializes_camel_case_and_defaults_missing_fields() {
    // The frontend sends camelCase keys; a partial payload must fall back to
    // per-field defaults (#[serde(rename_all = "camelCase", default)]).
    let cfg: OverlayConfig = serde_json::from_str(
        r##"{ "showProgress": true, "barFg": "#abcdef", "borderRadius": 12 }"##,
    )
    .expect("deserialize partial camelCase config");

    assert!(cfg.show_progress);
    assert_eq!(cfg.bar_fg, "#abcdef");
    assert_eq!(cfg.border_radius, 12);
    // Untouched fields keep their defaults.
    assert!(cfg.show_timer);
    assert_eq!(cfg.font_size, 2.0);
    assert_eq!(cfg.background, "transparent");
}

#[tokio::test]
async fn create_lists_groups_in_id_order() {
    let svc = OverlayService::new();
    let a = svc.create_group("Intro".into()).await.expect("create");
    let b = svc.create_group("BRB".into()).await.expect("create");

    let groups = svc.list_groups().await;
    assert_eq!(groups.len(), 2);
    assert_eq!(groups[0].id, a);
    assert_eq!(groups[1].id, b);
    assert_eq!(groups[0].name, "Intro");
}

#[tokio::test]
async fn blank_name_is_rejected() {
    let svc = OverlayService::new();
    assert!(svc.create_group("   ".into()).await.is_err());
}

#[tokio::test]
async fn update_sets_members_layout_and_hide_idle() {
    let svc = OverlayService::new();
    let id = svc.create_group("g".into()).await.expect("create");

    svc.update_group(id, "renamed".into(), vec![3, 1, 2], Layout::Row, true)
        .await
        .expect("update");

    let g = svc.get_group(id).await.expect("group exists");
    assert_eq!(g.name, "renamed");
    assert_eq!(g.members, vec![3, 1, 2]);
    assert_eq!(g.layout, Layout::Row);
    assert!(g.hide_idle);
}

#[tokio::test]
async fn delete_removes_group_and_errors_on_missing() {
    let svc = OverlayService::new();
    let id = svc.create_group("g".into()).await.expect("create");

    svc.delete_group(id).await.expect("delete");
    assert!(svc.get_group(id).await.is_none());
    assert!(svc.delete_group(id).await.is_err());
}

#[tokio::test]
async fn config_round_trips_and_defaults_when_unset() {
    let svc = OverlayService::new();
    // Unset config falls back to defaults so template rendering never sees a hole.
    let default = svc.get_config(42).await;
    assert!(default.show_timer);

    let mut cfg = default.clone();
    cfg.icon = "owl.svg".into();
    svc.set_config(42, cfg).await;
    assert_eq!(svc.get_config(42).await.icon, "owl.svg");
}

// --- 001 + 002 persist support (docs/001 + docs/002) -------------------

fn dto(
    id: u64,
    name: &str,
    members: Vec<u64>,
    layout: Layout,
    hide_idle: bool,
) -> GroupDto {
    GroupDto {
        id,
        name: name.to_string(),
        members,
        layout,
        hide_idle,
    }
}

/// Restore must rebuild the live service from DTOs and preserve every field:
/// group name, members (in order), layout, hide_idle. Per-countdown configs
/// must round-trip by id.
#[tokio::test]
async fn overlay_restore_round_trips_groups_and_configs() {
    let groups = vec![
        dto(2, "BRB", vec![11, 12], Layout::Row, true),
        dto(5, "Intro", vec![12], Layout::Column, false),
    ];
    let mut cfgs = std::collections::HashMap::new();
    let mut cfg12 = OverlayConfig::default();
    cfg12.icon = "owl.svg".into();
    cfg12.border_radius = 14;
    cfgs.insert(12, cfg12);

    let svc = OverlayService::from_groups_and_configs(groups, cfgs.clone());

    let listed = svc.list_groups().await;
    assert_eq!(listed.len(), 2);
    // First id wins, second dropped — they share id 2 isn't possible here; the
    // dedup invariant is covered separately below.
    let brb = listed.iter().find(|g| g.id == 2).expect("BRB present");
    assert_eq!(brb.name, "BRB");
    assert_eq!(brb.members, vec![11, 12]);
    assert_eq!(brb.layout, Layout::Row);
    assert!(brb.hide_idle);

    let intro = listed.iter().find(|g| g.id == 5).expect("Intro present");
    assert_eq!(intro.name, "Intro");
    assert_eq!(intro.members, vec![12]);
    assert_eq!(intro.layout, Layout::Column);
    assert!(!intro.hide_idle);

    // Unset config still defaults.
    assert_eq!(svc.get_config(7).await.icon, "");
    // Set one survives.
    let restored = svc.get_config(12).await;
    assert_eq!(restored.icon, "owl.svg");
    assert_eq!(restored.border_radius, 14);
}

/// Restored groups must not collide with newly created ones — next_id is
/// `max(restored id) + 1`.
#[tokio::test]
async fn overlay_restore_next_id_avoids_collision() {
    let groups = vec![dto(3, "g", vec![], Layout::Column, false)];
    let svc = OverlayService::from_groups_and_configs(groups, Default::default());

    let new_id = svc.create_group("fresh".into()).await.expect("create");
    assert_eq!(new_id, 4, "next_id must be max(restored id)+1");
}

/// `next_id` must not panic on the largest possible id (corrupt/hand-edited
/// store would have hit an overflow otherwise).
#[tokio::test]
async fn overlay_restore_next_id_does_not_overflow_on_max() {
    let groups = vec![dto(u64::MAX, "g", vec![], Layout::Column, false)];
    let svc = OverlayService::from_groups_and_configs(groups, Default::default());
    assert_eq!(svc.list_groups().await.len(), 1);
}

/// Duplicate ids in a hand-edited store must collapse to one entry (first
/// wins) so a corrupt save can't silently drop a group in nondeterministic
/// HashMap order.
#[tokio::test]
async fn overlay_restore_dedups_duplicate_group_ids() {
    let mut a = dto(7, "first", vec![1], Layout::Column, false);
    let b = dto(7, "second", vec![2], Layout::Row, true);
    a.layout = Layout::Row;
    let svc = OverlayService::from_groups_and_configs(vec![a, b], Default::default());

    let listed = svc.list_groups().await;
    assert_eq!(listed.len(), 1);
    assert_eq!(listed[0].name, "first");
    assert_eq!(listed[0].layout, Layout::Row);
}

/// Empty restore (first launch, corrupt fallback) starts ids at 0, matching
/// the fresh-service default.
#[tokio::test]
async fn overlay_restore_empty_starts_ids_at_zero() {
    let svc =
        OverlayService::from_groups_and_configs(Vec::new(), Default::default());
    assert!(svc.list_groups().await.is_empty());
    let new_id = svc
        .create_group("first".into())
        .await
        .expect("create");
    assert_eq!(new_id, 0);
}

/// `snapshot()` returns the current state shaped for the on-disk DTO so the
/// save path stays decoupled from service internals.
#[tokio::test]
async fn overlay_snapshot_returns_groups_and_configs() {
    let svc = OverlayService::new();
    let gid = svc.create_group("g".into()).await.expect("create");
    let mut cfg = OverlayConfig::default();
    cfg.icon = "owl.svg".into();
    svc.set_config(99, cfg).await;

    let (groups, cfgs) = svc.snapshot().await;
    assert_eq!(groups.len(), 1);
    assert_eq!(groups[0].id, gid);
    assert_eq!(groups[0].name, "g");
    assert_eq!(cfgs.len(), 1);
    assert_eq!(cfgs.get(&99).unwrap().icon, "owl.svg");
}

/// `GroupsAndConfigsDto` round-trips through JSON so a hand-edited file can
/// be loaded unchanged, and so on-disk shape stays stable across saves.
#[test]
fn overlay_dto_json_round_trip() {
    let groups = vec![dto(1, "Intro", vec![7], Layout::Column, true)];
    let mut cfgs = std::collections::HashMap::new();
    let mut cfg = OverlayConfig::default();
    cfg.icon = "owl.svg".into();
    cfgs.insert(7, cfg);

    let dto = GroupsAndConfigsDto::from_parts(groups.iter().cloned(), &cfgs);
    let json = serde_json::to_string(&dto).expect("serialize");
    let back: GroupsAndConfigsDto = serde_json::from_str(&json).expect("parse");

    let (back_groups, back_cfgs) = back.into_parts();
    assert_eq!(back_groups.len(), 1);
    assert_eq!(back_groups[0].name, "Intro");
    assert_eq!(back_groups[0].members, vec![7]);
    assert!(back_groups[0].hide_idle);
    assert_eq!(back_cfgs.len(), 1);
    assert_eq!(back_cfgs.get(&7).unwrap().icon, "owl.svg");
}

/// A non-numeric config id (corrupt/hand-edited store) must drop the entry
/// instead of panicking on parse — mirrors the corrupt-quarantine
/// philosophy in the store.
#[test]
fn overlay_dto_drops_non_numeric_config_keys() {
    let dto: GroupsAndConfigsDto = serde_json::from_str(
        r##"{
            "groups": [],
            "configs": { "not-a-number": { "icon": "owl.svg" } }
        }"##,
    )
    .expect("parse");
    let (_, cfgs) = dto.into_parts();
    assert!(cfgs.is_empty());
}

/// The wrap-style `OverlayConfigMapEntry` newtype must serialize transparently
/// so configs land at `configs["<id>"]` and not `configs["<id>"] {"value": ...}`.
#[test]
fn overlay_config_map_entry_serializes_transparently() {
    let mut cfg = OverlayConfig::default();
    cfg.icon = "owl.svg".into();
    let entry = OverlayConfigMapEntry(cfg);
    let json = serde_json::to_string(&entry).expect("serialize");
    assert!(json.contains("\"icon\":\"owl.svg\""), "got: {json}");
    assert!(
        !json.contains("\"value\""),
        "transparent wrapper must not emit a `value` envelope: {json}"
    );
}
