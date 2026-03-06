use crate::countdown::events::AppEvent;
use crate::countdown::service::CountdownService;
use std::collections::HashMap;
use tokio::sync::broadcast;

#[derive(Clone, Debug)]
pub struct ClockAnchor {
    pub boot_instant: tokio::time::Instant,
    pub boot_epoch_ms: u128,
}

impl ClockAnchor {
    pub fn new() -> Self {
        Self {
            boot_instant: tokio::time::Instant::now(),
            boot_epoch_ms: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_millis(),
        }
    }

    pub fn instant_to_epoch_ms(&self, instant: tokio::time::Instant) -> u128 {
        if let Some(delta) = instant.checked_duration_since(self.boot_instant) {
            self.boot_epoch_ms + delta.as_millis()
        } else {
            let delta = self.boot_instant.duration_since(instant).as_millis();
            self.boot_epoch_ms.saturating_sub(delta)
        }
    }
}

#[derive(Debug, Clone)]
pub struct OverlayConfig {
    pub icon: String,
    pub text_color: String,
    pub bg_color: String,
}

impl Default for OverlayConfig {
    fn default() -> Self {
        Self {
            icon: String::new(),
            text_color: "white".to_string(),
            bg_color: "transparent".to_string(),
        }
    }
}

#[derive(Debug)]
pub struct AppState {
    pub clock_anchor: ClockAnchor,
    pub countdown_service: CountdownService,
    pub event_bus: broadcast::Sender<AppEvent>,
    pub overlay_configs: tokio::sync::Mutex<HashMap<u64, OverlayConfig>>,
}

impl AppState {
    pub fn new() -> Self {
        let (event_bus, _) = broadcast::channel(64);
        Self {
            clock_anchor: ClockAnchor::new(),
            countdown_service: CountdownService::new(),
            event_bus,
            overlay_configs: tokio::sync::Mutex::new(HashMap::new()),
        }
    }
}
