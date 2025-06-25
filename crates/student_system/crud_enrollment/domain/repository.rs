use async_trait::async_trait;
use crate::enrollment::domain::{EnrollmentId, EnrollmentStatus};
use super::crud_enrollment::CrudEnrollment;

#[async_trait]
pub trait CrudEnrollmentRepository: Send + Sync {
    
    async fn find_all_by_student(&self, student_id: &str) -> Vec<CrudEnrollment>;
    async fn find_enrollment_id(&self, student_id: &str, course_id: &str) -> Option<EnrollmentId>;
    async fn update_status(&self, id: &EnrollmentId, new_status: EnrollmentStatus,) -> Result<(), String>;
}
