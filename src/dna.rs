//! Utility module for toon and doodle DNA

use std::fmt::Display;

use serde::Deserialize;

/// String that represents a toon or doodle's DNA.
#[derive(Deserialize, Debug, Clone)]
pub struct DNA(pub String);

impl Display for DNA {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}