use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct UserCode(pub String);

impl UserCode {
    pub fn new(code: String) -> Self {
        Self(code)
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for UserCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}