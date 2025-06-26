use crate::crud_enrollment::domain::*;
use crate::enrollment::domain::*;
use super::super::use_cases::*;
use crate::crud_enrollment::domain::repository::CrudEnrollmentRepository;
use crate::crud_enrollment::infrastructure::use_cases::repository::enrollment_dto::EnrolledCourseDto;

pub struct SupabaseCrudEnrollmentRepository {
    pub db: sea_orm::DatabaseConnection,
}

impl SupabaseCrudEnrollmentRepository {
    pub fn new(db: sea_orm::DatabaseConnection) -> Self {
        Self { db }
    }
}

#[async_trait::async_trait]
impl CrudEnrollmentRepository for &SupabaseCrudEnrollmentRepository {
    async fn get_schedule(&self, student_id: &str) -> Result<Vec<EnrolledCourseDto>, sea_orm::DbErr> {
        get_schedule::get_schedule(&self.db, student_id).await
    }

    async fn update_status(&self, id: &EnrollmentId, new_status: EnrollmentStatus) -> Result<(), String> {
        update_status::update_status(&self.db, id, new_status).await
    }
}