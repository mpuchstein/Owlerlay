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
    #[error("Id not found")]
    IdNotFound,
    #[error("Label not found")]
    LabelNotFound,
    #[error("Invalid duration")]
    InvalidDuration,
    #[error("Invalid state")]
    InvalidState,
    #[error("Invalid action")]
    InvalidAction,
    #[error("Max countdowns reached")]
    MaxCountdownsReached,
}
