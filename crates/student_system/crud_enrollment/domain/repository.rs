use async_trait::async_trait;
use crate::enrollment::domain::{EnrollmentId, EnrollmentStatus};
use crate::crud_enrollment::domain::enrollment_dto::EnrolledCourseDto;

#[async_trait]
pub trait CrudEnrollmentRepository: Send + Sync {
    async fn get_schedule(&self, student_id: &str) -> Result<Vec<EnrolledCourseDto>, sea_orm::DbErr>;
    async fn update_status(&self, id: &EnrollmentId, new_status: EnrollmentStatus) -> Result<(), String>;
}
