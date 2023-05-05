use std::error::Error as StdError;
use thiserror::Error;

/// Error encountered or produced by the Runtime.
/// This enum represents the faith of the errors produced during the state
/// transitions.
#[derive(Error, Debug)]
pub enum RuntimeError {
    /// Errors that need the runtime to try again without changing its state.
    #[error("An error occured: {message}. This runtime cycle will be skipped. Nested error: {nested_error:#?}.")]
    KeepState {
        /// error message
        message: String,

        /// Eventual caught error
        nested_error: Option<Box<dyn StdError + Sync + Send>>,
    },
    /// A Critical error means the Runtime stops and the software exits with an
    /// error code.
    #[error("Critical error:'{message}'. Nested error: {nested_error:#?}.")]
    Critical {
        /// error message
        message: String,

        /// Eventual caught error
        nested_error: Option<Box<dyn StdError + Sync + Send>>,
    },
}

impl RuntimeError {
    /// Create a new KeepState error
    pub fn keep_state(message: &str, error: Option<Box<dyn StdError + Sync + Send>>) -> Self {
        Self::KeepState {
            message: message.to_string(),
            nested_error: error,
        }
    }

    /// Create a new Critical error
    pub fn critical(message: &str, error: Option<Box<dyn StdError + Sync + Send>>) -> Self {
        Self::Critical {
            message: message.to_string(),
            nested_error: error,
        }
    }
}

impl From<Box<dyn StdError + Sync + Send>> for RuntimeError {
    fn from(value: Box<dyn StdError + Sync + Send>) -> Self {
        Self::KeepState {
            message: "Error caught, state preserved, will retry to cycle.".to_string(),
            nested_error: Some(value),
        }
    }
}

/// Errors returned when the runner cannot fulfil its missions with no subsystem
/// to fail.
#[derive(Debug, Error)]
// TODO: Are these errors still relevant, do we need to remove them?
#[allow(clippy::enum_variant_names)]
pub enum RunnerError {
    /// No stack distribution found
    #[error("Missing stack distribution: '{0}'.")]
    MissingStakeDistribution(String),

    /// Missing protocol parameters
    #[error("Missing protocol parameters: '{0}'.")]
    MissingProtocolParameters(String),
}
