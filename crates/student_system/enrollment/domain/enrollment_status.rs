#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EnrollmentStatus {
    Enrolled,
    Dropped,
    Completed,
}

impl EnrollmentStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            EnrollmentStatus::Enrolled => "enrolled",
            EnrollmentStatus::Dropped => "dropped",
            EnrollmentStatus::Completed => "completed",
        }
    }
}

impl std::str::FromStr for EnrollmentStatus {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "enrolled" => Ok(Self::Enrolled),
            "dropped" => Ok(Self::Dropped),
            "completed" => Ok(Self::Completed),
            _ => Err(()),
        }
    }
}
