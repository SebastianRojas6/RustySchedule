use std::convert::TryFrom;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SemesterParity {
    Even,
    Odd,
}

impl SemesterParity {
    pub fn from_u8(value: u8) -> Self {
        if value % 2 == 0 {
            SemesterParity::Even
        } else {
            SemesterParity::Odd
        }
    }
}

impl TryFrom<&str> for SemesterParity {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "Even" => Ok(SemesterParity::Even),
            "Odd" => Ok(SemesterParity::Odd),
            _ => Err(()),
        }
    }
}
