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

impl CountdownSnapshotDto {
    pub fn new(
        id: u64,
        label: String,
        duration: u128,
        state: CountdownState,
        start_epoch_ms: Option<u128>,
        target_epoch_ms: Option<u128>,
    ) -> Self {
        Self {
            id,
            label,
            duration,
            state,
            start_epoch_ms,
            target_epoch_ms,
        }
    }
}
