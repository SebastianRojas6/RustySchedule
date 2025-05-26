use std::convert::TryFrom;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CourseSection(pub u8);

impl CourseSection {
    pub fn new(section: u8) -> Result<Self, String> {
        if section == 0 || section > 3 {
            Err("Course section must be 1, 2, or 3".to_string())
        } else {
            Ok(Self(section))
        }
    }

    pub fn value(&self) -> u8 {
        self.0
    }
}

impl TryFrom<u8> for CourseSection {
    type Error = String;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        CourseSection::new(value)
    }
}