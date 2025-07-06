use super::facilitie::Facility;
use crate::domain::models::enums::Weekday;
use chrono::NaiveTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FacilityAvailable {
    pub facility: Facility,
    pub weekday: Weekday,
    pub start_time: NaiveTime,
    pub end_time: NaiveTime,
}
