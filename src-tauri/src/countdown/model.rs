use serde::{Deserialize, Serialize};
use tokio::time::{Duration, Instant};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Hash)]
pub enum CountdownState {
    Idle,
    Running,
    Paused,
    Finished,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CountdownError {
    InvalidTransition {
        from: CountdownState,
        action: &'static str,
    },
    TimeOverflow,
}

#[derive(Debug, Clone)]
pub struct Countdown {
    pub id: u64,
    pub label: String,
    initial_duration: Duration,
    remaining_duration_stored: Option<Duration>,
    state: CountdownState,
    start_timestamp: Option<Instant>,
    target_timestamp: Option<Instant>,
}

impl Countdown {
    pub fn new(id: u64, label: impl Into<String>, duration: Duration) -> Self {
        Self {
            id,
            label: label.into(),
            initial_duration: duration,
            remaining_duration_stored: None,
            state: CountdownState::Idle,
            start_timestamp: None,
            target_timestamp: None,
        }
    }

    pub fn id(&self) -> u64 {
        self.id
    }

    pub fn label(&self) -> &str {
        &self.label
    }

    pub fn state(&self) -> CountdownState {
        self.state
    }

    pub fn initial_duration(&self) -> Duration {
        self.initial_duration
    }

    pub fn start_timestamp(&self) -> Option<Instant> {
        self.start_timestamp
    }

    pub fn target_timestamp(&self) -> Option<Instant> {
        self.target_timestamp
    }

    pub fn remaining_at(&self, now: Instant) -> Duration {
        match self.state {
            CountdownState::Idle => self.initial_duration,
            CountdownState::Running => self
                .target_timestamp
                .map(|target| target.saturating_duration_since(now))
                .unwrap_or(self.initial_duration),
            CountdownState::Paused => self
                .remaining_duration_stored
                .unwrap_or(self.initial_duration),
            CountdownState::Finished => Duration::from_secs(0),
        }
    }

    pub fn remaining(&self) -> Duration {
        self.remaining_at(Instant::now())
    }

    pub fn is_finished(&self) -> bool {
        self.state == CountdownState::Finished
    }

    pub fn start(&mut self, now: Instant) -> Result<(), CountdownError> {
        match self.state {
            CountdownState::Idle => {
                let target = now
                    .checked_add(self.initial_duration)
                    .ok_or(CountdownError::TimeOverflow)?;
                self.start_timestamp = Some(now);
                self.target_timestamp = Some(target);
                self.remaining_duration_stored = None;
                self.state = CountdownState::Running;
                Ok(())
            }
            CountdownState::Paused => {
                let remaining =
                    self.remaining_duration_stored
                        .ok_or(CountdownError::InvalidTransition {
                            from: self.state,
                            action: "start",
                        })?;

                if remaining.is_zero() {
                    self.mark_finished();
                    return Ok(());
                }

                let target = now
                    .checked_add(remaining)
                    .ok_or(CountdownError::TimeOverflow)?;
                self.start_timestamp = Some(now);
                self.target_timestamp = Some(target);
                self.remaining_duration_stored = None;
                self.state = CountdownState::Running;
                Ok(())
            }
            _ => Err(CountdownError::InvalidTransition {
                from: self.state,
                action: "start",
            }),
        }
    }

    pub fn pause(&mut self, now: Instant) -> Result<(), CountdownError> {
        if self.state != CountdownState::Running {
            return Err(CountdownError::InvalidTransition {
                from: self.state,
                action: "pause",
            });
        }

        let remaining = self.remaining_at(now);
        if remaining.is_zero() {
            self.mark_finished();
            return Ok(());
        }

        self.remaining_duration_stored = Some(remaining);
        self.start_timestamp = None;
        self.target_timestamp = None;
        self.state = CountdownState::Paused;
        Ok(())
    }

    pub fn resume(&mut self, now: Instant) -> Result<(), CountdownError> {
        if self.state != CountdownState::Paused {
            return Err(CountdownError::InvalidTransition {
                from: self.state,
                action: "resume",
            });
        }

        self.start(now)
    }

    pub fn reset(&mut self) {
        self.state = CountdownState::Idle;
        self.remaining_duration_stored = None;
        self.start_timestamp = None;
        self.target_timestamp = None;
    }

    pub fn sync_finished_at(&mut self, now: Instant) {
        if self.state == CountdownState::Running && self.remaining_at(now).is_zero() {
            self.mark_finished();
        }
    }

    fn mark_finished(&mut self) {
        self.state = CountdownState::Finished;
        self.remaining_duration_stored = Some(Duration::from_secs(0));
        self.start_timestamp = None;
        self.target_timestamp = None;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn start_from_idle_enters_running() {
        let now = Instant::now();
        let mut countdown = Countdown::new(1, "test", Duration::from_secs(10));
        countdown.start(now).expect("start should succeed");

        assert_eq!(countdown.state(), CountdownState::Running);
        assert_eq!(countdown.remaining_at(now), Duration::from_secs(10));
    }

    #[test]
    fn pause_and_resume_preserve_remaining_time() {
        let now = Instant::now();
        let mut countdown = Countdown::new(1, "test", Duration::from_secs(10));
        countdown.start(now).expect("start should succeed");

        let after_three_seconds = now.checked_add(Duration::from_secs(3)).unwrap();
        countdown
            .pause(after_three_seconds)
            .expect("pause should succeed");
        assert_eq!(countdown.state(), CountdownState::Paused);
        assert_eq!(
            countdown.remaining_at(after_three_seconds),
            Duration::from_secs(7)
        );

        let resume_time = after_three_seconds
            .checked_add(Duration::from_secs(1))
            .unwrap();
        countdown
            .resume(resume_time)
            .expect("resume should succeed");
        assert_eq!(countdown.state(), CountdownState::Running);
        assert_eq!(countdown.remaining_at(resume_time), Duration::from_secs(7));
    }

    #[test]
    fn invalid_transition_returns_error() {
        let now = Instant::now();
        let mut countdown = Countdown::new(1, "test", Duration::from_secs(5));

        let err = countdown
            .pause(now)
            .expect_err("pause should fail from idle");
        assert_eq!(
            err,
            CountdownError::InvalidTransition {
                from: CountdownState::Idle,
                action: "pause",
            }
        );
    }

    #[test]
    fn running_countdown_reaches_finished() {
        let now = Instant::now();
        let mut countdown = Countdown::new(1, "test", Duration::from_secs(2));
        countdown.start(now).expect("start should succeed");

        let after_two_seconds = now.checked_add(Duration::from_secs(2)).unwrap();
        countdown.sync_finished_at(after_two_seconds);

        assert_eq!(countdown.state(), CountdownState::Finished);
        assert_eq!(
            countdown.remaining_at(after_two_seconds),
            Duration::from_secs(0)
        );
        assert!(countdown.is_finished());
    }
}
