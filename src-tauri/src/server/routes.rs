use crate::app_state::AppState;
use crate::countdown::commands::build_snapshot_dtos;
use crate::countdown::events::AppEvent;
use axum::extract::{Path, Query, State};
use axum::response::sse::KeepAlive;
use axum::response::{
    sse::{Event, Sse},
    Html,
};
use axum::Json;
use std::sync::Arc;
use std::time::Duration;
use tokio_stream::wrappers::BroadcastStream;
use tokio_stream::StreamExt;

#[derive(serde::Deserialize)]
pub struct OverlayQuery {
    id: u64,
}

pub async fn overlay_countdown(
    State(state): State<Arc<AppState>>,
    Query(q): Query<OverlayQuery>,
) -> Html<String> {
    let config = state
        .overlay_configs
        .lock()
        .await
        .get(&q.id)
        .cloned()
        .unwrap_or_default();
    let remaining = build_snapshot_dtos(&state)
        .await
        .ok()
        .and_then(|snaps| snaps.into_iter().find(|s| s.id == q.id))
        .map(|s| format_remaining(s.duration as u64, config.show_hh_mm))
        .unwrap_or_else(|| format_unknown(config.show_hh_mm));

    let mut env = minijinja::Environment::new();
    env.add_template(
        "page",
        include_str!("../../templates/overlay/countdown.html.j2"),
    )
    .unwrap();
    let html = env
        .get_template("page")
        .unwrap()
        .render(minijinja::context! {
            id => q.id,
            remaining => remaining,
            icon => config.icon,
            text_color => config.text_color,
            bg_color => config.bg_color,
        })
        .unwrap();
    Html(html)
}

pub async fn sse_countdown(
    State(state): State<Arc<AppState>>,
    Path(id): Path<u64>,
) -> Sse<impl futures_core::Stream<Item = Result<Event, axum::Error>>> {
    let show_hh_mm = state
        .overlay_configs
        .lock()
        .await
        .get(&id)
        .map(|cfg| cfg.show_hh_mm)
        .unwrap_or(false);
    let rx = state.event_bus.subscribe();
    let stream = BroadcastStream::new(rx).filter_map(move |event| match event {
        Ok(AppEvent::Tick(p)) if p.id == id => Some(Ok(Event::default()
            .event("tick")
            .data(format_remaining(p.remaining_ms, show_hh_mm)))),
        Ok(AppEvent::Changed(snaps)) => snaps.iter().find(|s| s.id == id).map(|s| {
            Ok(Event::default()
                .event("tick")
                .data(format_remaining(s.duration as u64, show_hh_mm)))
        }),
        Ok(AppEvent::ConfigChanged(cid)) if cid == id => {
            Some(Ok(Event::default().event("reload").data("")))
        }
        _ => None,
    });
    Sse::new(stream).keep_alive(
        KeepAlive::new()
            .interval(Duration::from_secs(15))
            .text("ping"),
    )
}

fn format_remaining(ms: u64, show_hh_mm: bool) -> String {
    let total_seconds = ms / 1_000;
    if !show_hh_mm {
        return total_seconds.to_string();
    }
    let h = ms / 3_600_000;
    let m = (ms % 3_600_000) / 60_000;
    let s = (ms % 60_000) / 1_000;
    format!("{:02}:{:02}:{:02}", h, m, s)
}

fn format_unknown(show_hh_mm: bool) -> String {
    if show_hh_mm {
        "??:??:??".to_string()
    } else {
        "??".to_string()
    }
}

pub async fn list_icons() -> Json<Vec<String>> {
    let mut names = Vec::new();
    if let Ok(mut entries) = tokio::fs::read_dir("public/icons").await {
        while let Ok(Some(entry)) = entries.next_entry().await {
            let path = entry.path();
            if matches!(
                path.extension().and_then(|e| e.to_str()),
                Some("svg" | "png")
            ) {
                if let Some(filename) = path.file_name().and_then(|s| s.to_str()) {
                    names.push(filename.to_string());
                }
            }
        }
    }
    names.sort();
    Json(names)
}
