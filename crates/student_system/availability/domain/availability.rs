use chrono::NaiveTime;
use chrono::Timelike;

use crate::enrollment::infrastructure::entity::sea_orm_active_enums::DayType as DbDayType;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DayType {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday,
}

impl DayType {
    pub fn from_str(day: &str) -> Option<Self> {
        match day.to_lowercase().as_str() {
            "monday" => Some(DayType::Monday),
            "tuesday" => Some(DayType::Tuesday),
            "wednesday" => Some(DayType::Wednesday),
            "thursday" => Some(DayType::Thursday),
            "friday" => Some(DayType::Friday),
            "saturday" => Some(DayType::Saturday),
            "sunday" => Some(DayType::Sunday),
            _ => None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Availability {
    pub student_id: String,
    pub day: DayType,
    pub start_time: NaiveTime,
    pub end_time: NaiveTime,
}

impl Availability {
    pub fn validate(&self) -> Result<(), String> {
        if self.start_time >= self.end_time {
            return Err("start_time must be before end_time".into());
        }

        if self.start_time.minute() != 0 || self.end_time.minute() != 0 {
            return Err("Only full-hour times are allowed (minutes must be 00)".into());
        }

        Ok(())
    }
}

impl std::fmt::Display for DayType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let day_str = match self {
            DayType::Monday => "monday",
            DayType::Tuesday => "tuesday",
            DayType::Wednesday => "wednesday",
            DayType::Thursday => "thursday",
            DayType::Friday => "friday",
            DayType::Saturday => "saturday",
            DayType::Sunday => "sunday",
        };
        write!(f, "{}", day_str)
    }
}

impl From<DayType> for DbDayType {
    fn from(day: DayType) -> Self {
        match day {
            DayType::Monday => DbDayType::Monday,
            DayType::Tuesday => DbDayType::Tuesday,
            DayType::Wednesday => DbDayType::Wednesday,
            DayType::Thursday => DbDayType::Thursday,
            DayType::Friday => DbDayType::Friday,
            DayType::Saturday => DbDayType::Saturday,
            DayType::Sunday => DbDayType::Sunday,
        }
    }
}

