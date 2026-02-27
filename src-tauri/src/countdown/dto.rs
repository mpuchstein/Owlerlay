use crate::countdown::model::CountdownState;

#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub struct CountdownSnapshotDto {
    pub id: u64,
    pub label: String,
    pub duration: u128,
    pub state: CountdownState,
    pub start_epoch_ms: Option<u128>,
    pub target_epoch_ms: Option<u128>,
}
