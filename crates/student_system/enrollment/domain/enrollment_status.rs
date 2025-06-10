use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EnrollmentStatus {
    Enrolled,
    Dropped,
    Completed,
}

impl EnrollmentStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            EnrollmentStatus::Enrolled => "Enrolled",
            EnrollmentStatus::Dropped => "Dropped",
            EnrollmentStatus::Completed => "Completed",
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
            _ => Err(()),
        }
    }
}