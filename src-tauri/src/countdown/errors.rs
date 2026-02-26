use crate::countdown::model::CountdownState;
use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum CountdownError {
    #[error("Invalid transition from {from:?} to {action:?}")]
    InvalidTransition {
        from: CountdownState,
        action: &'static str,
    },
    #[error("Time overflow")]
    TimeOverflow,
}
