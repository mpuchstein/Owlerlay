use crate::countdown::errors::CountdownError;
use crate::countdown::model::{Countdown, CountdownState};
use tokio::sync::Mutex;
use tokio::time::{Duration, Instant};

#[derive(Debug)]
pub struct CountdownService {
    countdown: Mutex<Countdown>,
}

#[derive(Debug)]
pub struct CountdownSnapshot {
    pub id: u64,
    pub label: String,
    pub state: CountdownState,
    pub duration: Duration,
    pub start_instant: Option<Instant>,
    pub target_instant: Option<Instant>,
}

impl CountdownService {
    pub fn default() -> Self {
        Self::new(0, "Countdown0", Duration::new(600, 0))
    }

    pub fn new(id: u64, label: &str, duration: Duration) -> Self {
        Self {
            countdown: Mutex::new(Countdown::new(id, label, duration)),
        }
    }

    pub async fn snapshot(&self, now: Instant) -> CountdownSnapshot {
        let countdown = self.countdown.lock().await.clone();
        CountdownSnapshot {
            id: countdown.id(),
            label: countdown.label().to_string(),
            state: countdown.state(),
            duration: countdown.remaining_at(now),
            start_instant: countdown.start_timestamp(),
            target_instant: countdown.target_timestamp(),
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
