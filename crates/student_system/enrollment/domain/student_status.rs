use super::CreditAmount;
use std::convert::TryFrom;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StudentStatus {
    Regular,
    Observed,
}

impl StudentStatus {
    pub fn is_regular(&self) -> bool {
        matches!(self, StudentStatus::Regular)
    }

    pub fn is_observed(&self) -> bool {
        matches!(self, StudentStatus::Observed)
    }
    
    pub fn max_credits(&self) -> u8 {
        match self {
            StudentStatus::Regular => CreditAmount::MAX_CREDITS_REGULAR,
            StudentStatus::Observed => CreditAmount::MAX_CREDITS_OBSERVED,
        }
    }
}

impl TryFrom<&str> for StudentStatus {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "Regular" => Ok(StudentStatus::Regular),
            "Observed" => Ok(StudentStatus::Observed),
            _ => Err(()),
        }
    }
}