use serde::{Serialize, Deserialize};
use std::fmt;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct EnrollmentId(String);

impl EnrollmentId {
    pub fn new(id: String) -> Self {
        assert!(!id.trim().is_empty(), "Bro, no puede ser vacío");
        Self(id)
    }

    pub fn value(&self) -> &str {
        &self.0
    }
    
    pub fn generate() -> Self {
        Self(Uuid::new_v4().to_string())
    }
}

impl fmt::Display for EnrollmentId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}