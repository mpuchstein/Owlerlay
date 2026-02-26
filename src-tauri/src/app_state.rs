use crate::countdown::service::CountdownService;
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
        instant.duration_since(self.boot_instant).as_millis() + self.boot_epoch_ms
    }
}
//TODO: implement the handling of multiple countdowns
#[derive(Debug)]
pub struct AppState {
    pub clock_anchor: ClockAnchor,
    pub countdown_service: CountdownService,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            clock_anchor: ClockAnchor::new(),
            countdown_service: CountdownService::default(),
        }
    }
}
