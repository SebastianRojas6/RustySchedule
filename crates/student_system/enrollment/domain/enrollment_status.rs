use std::fmt;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum EnrollmentStatus {
    Enrolled,
    Dropped,
    Completed,
    Withdrawn,
    Failed,
    Pending,
}

impl EnrollmentStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            EnrollmentStatus::Enrolled => "Enrolled",
            EnrollmentStatus::Dropped => "Dropped",
            EnrollmentStatus::Completed => "Completed",
            EnrollmentStatus::Withdrawn => "Withdrawn",
            EnrollmentStatus::Failed => "Failed",
            EnrollmentStatus::Pending => "Pending",
        }
    }
}

impl fmt::Display for EnrollmentStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl std::str::FromStr for EnrollmentStatus {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "Enrolled" => Ok(EnrollmentStatus::Enrolled),
            "Dropped" => Ok(EnrollmentStatus::Dropped),
            "Completed" => Ok(EnrollmentStatus::Completed),
            "Withdrawn" => Ok(EnrollmentStatus::Withdrawn),
            "Failed" => Ok(EnrollmentStatus::Failed),
            "Pending" => Ok(EnrollmentStatus::Pending),
            _ => Err(()),
        }
    }
}