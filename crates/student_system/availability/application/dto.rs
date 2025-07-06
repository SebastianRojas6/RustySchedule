use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct TimeSlotDto {
    pub day: String,         
    pub start_time: String,  
    pub end_time: String,    
}

#[derive(Debug, Deserialize, Clone)]
pub struct AvailabilityRequest {
    pub availabilities: Vec<TimeSlotDto>,
}
