//! Order state machine module
//!
//! This module implements a state machine for managing order lifecycle transitions.
//! Orders follow a specific state transition flow to ensure data consistency.

use crate::core::trade::types::OrderState;
use anyhow::{bail, Result};

/// Order state machine that enforces valid state transitions
#[derive(Debug, Clone)]
pub struct OrderStateMachine {
    state: OrderState,
}

impl OrderStateMachine {
    /// Create a new order state machine in Pending state
    pub fn new() -> Self {
        Self {
            state: OrderState::Pending,
        }
    }

    /// Create a state machine from an existing state
    pub fn from_state(state: OrderState) -> Self {
        Self { state }
    }

    /// Transition to a new state if the transition is valid
    ///
    /// # Valid Transitions
    /// - Pending -> Open
    /// - Pending -> Rejected
    /// - Open -> PartiallyFilled
    /// - Open -> Filled
    /// - Open -> Canceled
    /// - PartiallyFilled -> Filled
    /// - PartiallyFilled -> Canceled
    ///
    /// # Errors
    /// Returns an error if the transition is invalid
    pub fn transition_to(&mut self, new_state: OrderState) -> Result<()> {
        self.validate_transition(&self.state, &new_state)?;
        self.state = new_state;
        Ok(())
    }

    /// Validate if a state transition is valid
    fn validate_transition(&self, from: &OrderState, to: &OrderState) -> Result<()> {
        // Same state is allowed (idempotent transition)
        if from == to {
            return Ok(());
        }

        match (from, to) {
            // From Pending state
            (OrderState::Pending, OrderState::Open) => Ok(()),
            (OrderState::Pending, OrderState::Rejected) => Ok(()),

            // From Open state
            (OrderState::Open, OrderState::PartiallyFilled) => Ok(()),
            (OrderState::Open, OrderState::Filled) => Ok(()),
            (OrderState::Open, OrderState::Canceled) => Ok(()),

            // From PartiallyFilled state
            (OrderState::PartiallyFilled, OrderState::Filled) => Ok(()),
            (OrderState::PartiallyFilled, OrderState::Canceled) => Ok(()),

            // All other transitions are invalid
            _ => bail!("Invalid state transition: {:?} -> {:?}", from, to),
        }
    }

    /// Get the current state
    pub fn state(&self) -> OrderState {
        self.state
    }

    /// Check if the order is in a terminal state
    pub fn is_terminal(&self) -> bool {
        self.state.is_terminal()
    }

    /// Check if the order is active
    pub fn is_active(&self) -> bool {
        self.state.is_active()
    }

    /// Check if a transition to the target state is valid without performing it
    pub fn can_transition_to(&self, target_state: &OrderState) -> bool {
        self.validate_transition(&self.state, target_state).is_ok()
    }
}

impl Default for OrderStateMachine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_state_machine() {
        let sm = OrderStateMachine::new();
        assert_eq!(sm.state(), OrderState::Pending);
        assert!(!sm.is_terminal());
        assert!(!sm.is_active());
    }

    #[test]
    fn test_valid_transitions() {
        let mut sm = OrderStateMachine::new();

        // Pending -> Open
        assert!(sm.transition_to(OrderState::Open).is_ok());
        assert_eq!(sm.state(), OrderState::Open);
        assert!(sm.is_active());

        // Open -> PartiallyFilled
        assert!(sm.transition_to(OrderState::PartiallyFilled).is_ok());
        assert_eq!(sm.state(), OrderState::PartiallyFilled);
        assert!(sm.is_active());

        // PartiallyFilled -> Filled
        assert!(sm.transition_to(OrderState::Filled).is_ok());
        assert_eq!(sm.state(), OrderState::Filled);
        assert!(sm.is_terminal());
    }

    #[test]
    fn test_pending_to_rejected() {
        let mut sm = OrderStateMachine::new();
        assert!(sm.transition_to(OrderState::Rejected).is_ok());
        assert_eq!(sm.state(), OrderState::Rejected);
        assert!(sm.is_terminal());
    }

    #[test]
    fn test_open_to_canceled() {
        let mut sm = OrderStateMachine::new();
        sm.transition_to(OrderState::Open).unwrap();
        assert!(sm.transition_to(OrderState::Canceled).is_ok());
        assert_eq!(sm.state(), OrderState::Canceled);
        assert!(sm.is_terminal());
    }

    #[test]
    fn test_open_directly_to_filled() {
        let mut sm = OrderStateMachine::new();
        sm.transition_to(OrderState::Open).unwrap();
        assert!(sm.transition_to(OrderState::Filled).is_ok());
        assert_eq!(sm.state(), OrderState::Filled);
        assert!(sm.is_terminal());
    }

    #[test]
    fn test_invalid_transitions() {
        let mut sm = OrderStateMachine::new();

        // Cannot transition from Pending to Filled directly
        assert!(sm.transition_to(OrderState::Filled).is_err());

        // Cannot transition from Pending to Canceled directly
        assert!(sm.transition_to(OrderState::Canceled).is_err());

        // Cannot transition from Open to Pending
        sm.transition_to(OrderState::Open).unwrap();
        assert!(sm.transition_to(OrderState::Pending).is_err());

        // Cannot transition from Filled (terminal) to any other state
        sm.transition_to(OrderState::Filled).unwrap();
        assert!(sm.transition_to(OrderState::Open).is_err());
        assert!(sm.transition_to(OrderState::Canceled).is_err());
    }

    #[test]
    fn test_idempotent_transitions() {
        let mut sm = OrderStateMachine::new();

        // Transitioning to the same state should be allowed
        assert!(sm.transition_to(OrderState::Pending).is_ok());

        sm.transition_to(OrderState::Open).unwrap();
        assert!(sm.transition_to(OrderState::Open).is_ok());
    }

    #[test]
    fn test_can_transition_to() {
        let sm = OrderStateMachine::new();

        // From Pending, can go to Open or Rejected
        assert!(sm.can_transition_to(&OrderState::Open));
        assert!(sm.can_transition_to(&OrderState::Rejected));
        assert!(!sm.can_transition_to(&OrderState::Filled));
        assert!(!sm.can_transition_to(&OrderState::Canceled));
    }

    #[test]
    fn test_from_state() {
        let sm = OrderStateMachine::from_state(OrderState::Filled);
        assert_eq!(sm.state(), OrderState::Filled);
        assert!(sm.is_terminal());
        assert!(!sm.is_active());
    }

    #[test]
    fn test_display_state() {
        assert_eq!(OrderState::Pending.to_string(), "pending");
        assert_eq!(OrderState::Open.to_string(), "open");
        assert_eq!(OrderState::PartiallyFilled.to_string(), "partially_filled");
        assert_eq!(OrderState::Filled.to_string(), "filled");
        assert_eq!(OrderState::Canceled.to_string(), "canceled");
        assert_eq!(OrderState::Rejected.to_string(), "rejected");
    }

    #[test]
    fn test_from_str_state() {
        assert_eq!("pending".parse::<OrderState>().unwrap(), OrderState::Pending);
        assert_eq!("open".parse::<OrderState>().unwrap(), OrderState::Open);
        assert_eq!("partially_filled".parse::<OrderState>().unwrap(), OrderState::PartiallyFilled);
        assert_eq!("filled".parse::<OrderState>().unwrap(), OrderState::Filled);
        assert_eq!("canceled".parse::<OrderState>().unwrap(), OrderState::Canceled);
        assert_eq!("rejected".parse::<OrderState>().unwrap(), OrderState::Rejected);

        // Case insensitive
        assert_eq!("OPEN".parse::<OrderState>().unwrap(), OrderState::Open);
        assert_eq!("Filled".parse::<OrderState>().unwrap(), OrderState::Filled);

        // Invalid state
        assert!("invalid".parse::<OrderState>().is_err());
    }
}
