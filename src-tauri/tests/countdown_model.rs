use owlerlay_lib::countdown::errors::CountdownError;
use owlerlay_lib::countdown::model::{Countdown, CountdownState};
use tokio::time::{Duration, Instant};

#[test]
fn start_from_idle_enters_running() {
    let now = Instant::now();
    let mut countdown = Countdown::new("test", Duration::from_secs(10));
    countdown.start(now).expect("start should succeed");

    assert_eq!(countdown.state(), CountdownState::Running);
    assert_eq!(countdown.remaining_at(now), Duration::from_secs(10));
}

#[test]
fn pause_and_resume_preserve_remaining_time() {
    let now = Instant::now();
    let mut countdown = Countdown::new("test", Duration::from_secs(10));
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
    let mut countdown = Countdown::new("test", Duration::from_secs(5));

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
    let mut countdown = Countdown::new("test", Duration::from_secs(2));
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
