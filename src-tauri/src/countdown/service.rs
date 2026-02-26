use tokio::sync::Mutex;
use tokio::time::{Duration, Instant};

use crate::countdown::dto::CountdownSnapshotDto;
use crate::countdown::model::{Countdown, CountdownError};

#[derive(Debug)]
pub struct CountdownService {
    countdown: Mutex<Countdown>,
    next_id: u64,
}

impl CountdownService {
    pub fn new() -> Self {
        Self {
            countdown: Mutex::new(Countdown::new(0, "Countdown0", Duration::new(600, 0))),
            next_id: 1,
        }
    }

    pub async fn snapshot(&self, now: Instant) -> CountdownSnapshotDto {
        let countdown = self.countdown.lock().await;
        CountdownSnapshotDto {
            id: countdown.id,
            label: countdown.label.to_string(),
            state: countdown.state(),
            duration: countdown.remaining(),
            start_epoch_ms: Some(countdown.start_epoch_ms()),
            target_epoch_ms: Some(countdown.target_epoch_ms()),
        }
    }

    pub async fn start(&self, now: Instant) -> Result<(), CountdownError> {
        let mut countdown = self.countdown.lock().await;
        countdown.start(now)
    }

    pub async fn reset(&self) {
        let mut countdown = self.countdown.lock().await;
        countdown.reset()
    }

    pub async fn resume(&self, now: Instant) -> Result<(), CountdownError> {
        let mut countdown = self.countdown.lock().await;
        countdown.resume(now)
    }

    pub async fn pause(&self, now: Instant) -> Result<(), CountdownError> {
        let mut countdown = self.countdown.lock().await;
        countdown.pause(now)
    }
}
