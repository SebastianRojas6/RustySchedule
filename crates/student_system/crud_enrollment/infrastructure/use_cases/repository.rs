/*use crate::enrollment::domain::*;
use super::super::use_cases::*;

pub struct SupabaseCrudEnrollmentRepository {
    pub db: sea_orm::DatabaseConnection,
}

impl SupabaseCrudEnrollmentRepository {
    pub fn new(db: sea_orm::DatabaseConnection) -> Self {
        Self { db }
    }
}

#[async_trait::async_trait]
impl CrudEnrollmentRepository for SupabaseCrudEnrollmentRepository {
    async fn find_all_by_student(&self, student_id: &str) -> Vec<CrudEnrollment> {
        find_all_by_student::find_all_by_student(&self.db, student_id).await
    }

    async fn update_status(&self, id: &EnrollmentId, new_status: EnrollmentStatus) -> Result<(), String> {
        update_status::update_status(&self.db, id, new_status).await
    }

    async fn find_enrollment_id(&self, student_id: &str, course_id: &str) -> Option<EnrollmentId> {
    find_enrollment_id::find_enrollment_id(&self.db, student_id, course_id).await }

}
*/