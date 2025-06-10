use crate::enrollment::domain::*;
use super::super::use_cases::*;

pub struct SupabaseEnrollmentRepository {
    pub db: sea_orm::DatabaseConnection,
}

impl SupabaseEnrollmentRepository {
    pub async fn new() -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        let db = shared::config::connect_to_supabase().await?;
        Ok(Self { db })
    }
}

#[async_trait::async_trait]
impl EnrollmentRepository for SupabaseEnrollmentRepository {
    async fn find_by_id(&self, id: &EnrollmentId) -> Option<Enrollment> {
        find_by_id::find_by_id(&self.db, id).await
    }

    async fn save(&self, enrollment: &Enrollment) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        save::save(&self.db, enrollment).await
    }

    async fn delete(&self, _id: &EnrollmentId) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        unimplemented!()
    }

    async fn completed_courses(&self, _student_id: &UserId) -> Vec<CourseId> {
        unimplemented!()
    }

    async fn current_enrollments(&self, _student_id: &UserId) -> Vec<Enrollment> {
        unimplemented!()
    }

    async fn find_by_student_and_course(&self, _student_id: &UserId, _course_id: &CourseId) -> Option<Enrollment> {
        unimplemented!()
    }

    async fn total_credits_enrolled(&self, _student_id: &UserId) -> CreditAmount {
        unimplemented!()
    }

    async fn enrollment_attempts(&self, _student_id: &UserId, _course_id: &CourseId) -> u8 {
        unimplemented!()
    }
}
