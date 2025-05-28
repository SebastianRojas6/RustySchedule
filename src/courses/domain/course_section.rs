use std::convert::TryFrom;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CourseSection(pub i32);

impl CourseSection {
    pub fn new(section: i32) -> Result<Self, String> {
        if section == 0 || section > 3 {
            Err("Course section must be 1, 2, or 3".to_string())
        } else {
            Ok(Self(section))
        }
    }

    pub fn value(&self) -> i32 {
        self.0
    }
}

impl TryFrom<i32> for CourseSection {
    type Error = String;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        CourseSection::new(value)
    }
}