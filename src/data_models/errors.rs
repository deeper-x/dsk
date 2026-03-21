use core::fmt;
use std::env::VarError;

pub enum AIError {
    Env(VarError),
}

impl fmt::Display for AIError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AIError::Env(e) => write!(f, "Missing env variable {}", e),
        }
    }
}
