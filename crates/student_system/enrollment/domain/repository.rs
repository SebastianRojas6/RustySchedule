use async_trait::async_trait;
use super::super::domain::{Enrollment, EnrollmentId, UserId, CourseId, CreditAmount};

#[async_trait]
pub trait EnrollmentRepository: Send + Sync {
    async fn find_by_id(&self, id: &EnrollmentId) -> Option<Enrollment>;

    async fn find_by_student_and_course(&self, student_id: &UserId, course_id: &CourseId) -> Option<Enrollment>;

    async fn save(&self, enrollment: &Enrollment) -> Result<(), Box<dyn std::error::Error + Send + Sync>>;

    async fn delete(&self, id: &EnrollmentId) -> Result<(), Box<dyn std::error::Error + Send + Sync>>;

    async fn total_credits_enrolled(&self, student_id: &UserId) -> CreditAmount;

    async fn completed_courses(&self, student_id: &UserId) -> Vec<CourseId>;

    async fn enrollment_attempts(&self, student_id: &UserId, course_id: &CourseId) -> u8;

    async fn current_enrollments(&self, student_id: &UserId) -> Vec<Enrollment>;
}
