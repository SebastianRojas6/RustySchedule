use super::enums::Weekday;
use super::facilitie::Facility;
use chrono::NaiveTime;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FacilityAvailable {
    pub facility: Facility,
    pub available_slots: BTreeMap<Weekday, Vec<(NaiveTime, NaiveTime)>>,
}
