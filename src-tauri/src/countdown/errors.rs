use crate::countdown::model::CountdownState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CountdownError {
    InvalidTransition {
        from: CountdownState,
        action: &'static str,
    },
    TimeOverflow,
}
