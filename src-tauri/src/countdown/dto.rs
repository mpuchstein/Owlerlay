use tokio::time::{Duration, Instant};

use crate::countdown::model::CountdownState;

#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub struct CountdownSnapshotDto {
    pub id: u64,
    pub label: String,
    pub duration: Duration,
    pub state: CountdownState,
    pub start_epoch_ms: Option<u64>,
    pub target_epoch_ms: Option<u64>,
}

impl CountdownSnapshotDto {
    pub fn new(
        id: u64,
        label: String,
        duration: Duration,
        state: CountdownState,
        start_epoch_ms: Option<u64>,
        target_epoch_ms: Option<u64>,
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
