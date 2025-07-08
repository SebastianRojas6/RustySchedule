use chrono::NaiveTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct SectionId(pub String);

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct CourseCode(pub String);

impl From<crate::suggestion::domain::section::CourseCode>
    for crate::enrollment::domain::available_course::CourseCode
{
    fn from(code: crate::suggestion::domain::section::CourseCode) -> Self {
        Self(code.0)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Day {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct TimeSlot {
    pub day: Day,
    pub start_time: NaiveTime,
    pub end_time: NaiveTime,
}

impl TimeSlot {
    pub fn conflicts_with(&self, other: &TimeSlot) -> bool {
        self.day == other.day
            && !(self.end_time <= other.start_time || self.start_time >= other.end_time)
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Section {
    pub id: SectionId, 
    pub course_code: CourseCode,
    pub teacher_id: String,
    pub available_spots: u32,
    pub times: Vec<TimeSlot>,
}
