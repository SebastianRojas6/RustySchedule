use crate::availability::domain::availability::{Availability, DayType};
use crate::availability::domain::repository::EnrollmentAvailabilityRepository;
use crate::availability::application::dto::{AvailabilityRequest, TimeSlotDto};
use chrono::NaiveTime;
use std::sync::Arc;

pub struct RegisterAvailabilityUseCase {
    repo: Arc<dyn EnrollmentAvailabilityRepository>,
}

impl RegisterAvailabilityUseCase {
    pub fn new(repo: Arc<dyn EnrollmentAvailabilityRepository>) -> Self {
        Self { repo }
    }

    pub async fn execute(
        &self,
        student_id: &str,
        request: AvailabilityRequest,
    ) -> Result<(), String> {
        let mut availabilities = Vec::new();

        for TimeSlotDto {
            day,
            start_time,
            end_time,
        } in request.availabilities
        {
            let day_enum = DayType::from_str(&day)
                .ok_or_else(|| format!("Invalid day: {}", day))?;

            let start = NaiveTime::parse_from_str(&start_time, "%H:%M")
                .map_err(|_| format!("Invalid start_time: {}", start_time))?;

            let end = NaiveTime::parse_from_str(&end_time, "%H:%M")
                .map_err(|_| format!("Invalid end_time: {}", end_time))?;

            let availability = Availability {
                student_id: student_id.to_string(),
                day: day_enum,
                start_time: start,
                end_time: end,
            };

            availability.validate()?;

            availabilities.push(availability);
        }

        self.repo.delete_all_by_student(student_id).await?;
        self.repo.save_all(availabilities).await?;

        Ok(())
    }
}
