use crate::countdown::dto::CountdownSnapshotDto;
use crate::countdown::errors::CountdownError;
use crate::AppState;
use tauri::{command, State};
use tokio::time::Instant;

type CmdResult<T> = Result<T, String>;

#[command]
pub async fn countdown_start(state: State<'_, AppState>) -> Result<(), String> {
    state
        .countdown_service
        .start(Instant::now())
        .await
        .map_err(|e: CountdownError| format!("{e:?}"))?;
    Ok(())
}

#[command]
pub async fn countdown_reset(state: State<'_, AppState>) -> Result<(), String> {
    state.countdown_service.reset().await;
    Ok(())
}

#[command]
pub async fn countdown_pause(state: State<'_, AppState>) -> Result<(), String> {
    state
        .countdown_service
        .pause(Instant::now())
        .await
        .map_err(|e: CountdownError| format!("{e:?}"))?;
    Ok(())
}

#[command]
pub async fn countdown_resume(state: State<'_, AppState>) -> Result<(), String> {
    state
        .countdown_service
        .resume(Instant::now())
        .await
        .map_err(|e: CountdownError| format!("{e:?}"))?;
    Ok(())
}

#[command]
pub async fn countdown_snapshot(
    state: State<'_, AppState>,
) -> Result<CountdownSnapshotDto, String> {
    let countdown_snapshot = state.countdown_service.snapshot(Instant::now()).await;
    let start = match countdown_snapshot.start_instant {
        Some(instant) => Some(state.clock_anchor.instant_to_epoch_ms(instant)),
        None => None,
    };
    let target = match countdown_snapshot.target_instant {
        Some(instant) => Some(state.clock_anchor.instant_to_epoch_ms(instant)),
        None => None,
    };
    Ok(CountdownSnapshotDto {
        id: countdown_snapshot.id,
        label: countdown_snapshot.label,
        duration: countdown_snapshot.duration,
        state: countdown_snapshot.state,
        start_epoch_ms: start,
        target_epoch_ms: target,
    })
}
