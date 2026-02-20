use std::time::{Instant, Duration}

enum CountdownState{
    Idle,
    Running,
    Paused,
    Finished
}

#[derive(Debug)]
struct Countdown {
    id: uint,
    label: &str,
    initial_duration: Duration,
    remaining_duration_stored: Option<Duration>,
    state: CountdownState,
    start_timestamp: Option<Instant>,
    target_timestamp: Option<Instant>,
}

impl Countdown{
    
    fn create(&self, id: uint, label: &str, duration: Duration) -> Option<Countdown>{
        self.id = id;
        self.label = label;
        self.initial_duration = duration;
        self.state = CountdownState::Idle;
        self.remaining_duration_stored = None;
        self.start_timestamp = None;
        self.target_timestamp = None;
    }

    fn remaining_at(&self, timestamp: Instant) -> Duration{
        timestamp.saturating_duration_since(target_timestamp)
    }

    fn remaining(&self) -> Duration{
        self.remaining_at(Instant::Now())
    }

    fn is_finished(&self) -> bool {
        self.state == CountdownState::Finished
    }

    fn start(&self) -> Result {
        match self.state{
            CountdownState::Idle => {
                start_timestamp = Instant::Now();
                target_timestamp = start.timestamp.checked_add(self.initial_duration).unwrap();
            },
            CountdownState::Paused => {
                target_timestamp = Instant::Now().checked_add(self.remaining_duration_stored).unwrap();
            },
            _ => Err("not startable")
        }
        self.remaining_duration_stored = None;
        self.State = Running;
        Ok()
    }

    fn pause(&self) -> Result {
        match self.state {
            CountdownState::Running => {
                self.remaining_duration_stored = Instant::Now().saturating_duration_since(self.target_timestamp);
                self.state = CountdownState::Paused;
                Ok()
            },
            _ => Err("not pausable")
        }
    }

    fn reset(&self) -> Result{
        self.state = CountdownState::Idle;
        self.remaining_duration_stored = None;
        self.start_timestamp = None;
        self.target_timestamp = None;
        Ok()
    }

}
