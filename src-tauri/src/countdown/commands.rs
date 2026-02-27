use crate::countdown::dto::CountdownSnapshotDto;
use crate::countdown::errors::CountdownError;
use crate::AppState;
use tauri::{command, State};
use tokio::time::{Duration, Instant};

#[command]
pub async fn countdown_create(
    state: State<'_, AppState>,
    label: String,
    duration: u64,
) -> Result<u64, String> {
    let duration = Duration::from_millis(duration);
    state
        .countdown_service
        .create_countdown(label, duration)
        .await
        .map_err(|e: CountdownError| e.to_string())
}

#[command]
pub async fn countdown_list(
    state: State<'_, AppState>,
) -> Result<Vec<CountdownSnapshotDto>, String> {
    let snapshots = state
        .countdown_service
        .list_countdown()
        .await
        .map_err(|e: CountdownError| e.to_string())?;
    let mut snapshot_dtos: Vec<CountdownSnapshotDto> = Vec::new();
    for snapshot in snapshots {
        let start = match snapshot.start_instant {
            Some(instant) => Some(state.clock_anchor.instant_to_epoch_ms(instant)),
            None => None,
        };
        let target = match snapshot.target_instant {
            Some(instant) => Some(state.clock_anchor.instant_to_epoch_ms(instant)),
            None => None,
        };
        snapshot_dtos.push(CountdownSnapshotDto {
            id: snapshot.id,
            label: snapshot.label,
            duration: snapshot.duration.as_millis(),
            state: snapshot.state,
            start_epoch_ms: start,
            target_epoch_ms: target,
        })
    }
    Ok(snapshot_dtos)
}

#[command]
pub async fn countdown_delete(state: State<'_, AppState>, id: u64) -> Result<(), String> {
    state
        .countdown_service
        .delete_countdown(id)
        .await
        .map_err(|e: CountdownError| e.to_string())?;
    Ok(())
}

#[command]
pub async fn countdown_start(state: State<'_, AppState>, id: u64) -> Result<(), String> {
    state
        .countdown_service
        .start(id, Instant::now())
        .await
        .map_err(|e: CountdownError| e.to_string())?;
    Ok(())
}

#[command]
pub async fn countdown_reset(state: State<'_, AppState>, id: u64) -> Result<(), String> {
    state
        .countdown_service
        .reset(id)
        .await
        .map_err(|e: CountdownError| e.to_string())?;
    Ok(())
}

#[command]
pub async fn countdown_pause(state: State<'_, AppState>, id: u64) -> Result<(), String> {
    state
        .countdown_service
        .pause(id, Instant::now())
        .await
        .map_err(|e: CountdownError| e.to_string())?;
    Ok(())
}

#[command]
pub async fn countdown_resume(state: State<'_, AppState>, id: u64) -> Result<(), String> {
    state
        .countdown_service
        .resume(id, Instant::now())
        .await
        .map_err(|e: CountdownError| e.to_string())?;
    Ok(())
}

#[command]
pub async fn countdown_snapshot(
    state: State<'_, AppState>,
    id: u64,
) -> Result<CountdownSnapshotDto, String> {
    let countdown_snapshot = state
        .countdown_service
        .snapshot(id, Instant::now())
        .await
        .map_err(|e: CountdownError| e.to_string())?;
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
        duration: countdown_snapshot.duration.as_millis(),
        state: countdown_snapshot.state,
        start_epoch_ms: start,
        target_epoch_ms: target,
    })
}
