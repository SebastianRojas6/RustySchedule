use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct CourseId(pub String);

impl CourseId {
    pub fn new(id: String) -> Self {
        Self(id)
    }

    pub fn value(&self) -> &str {
        &self.0
    }

}

impl fmt::Display for CourseId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}