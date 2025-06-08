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
