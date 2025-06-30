use async_trait::async_trait;
use super::super::domain::{Enrollment, EnrollmentId, UserId, CourseId, CreditAmount, UserCode};

#[async_trait]
pub trait EnrollmentRepository: Send + Sync {
    async fn find_by_id(&self, id: &EnrollmentId) -> Option<Enrollment>;

    async fn find_by_student_and_course(&self, student_id: &UserId, course_id: &CourseId) -> Option<Enrollment>;

    async fn total_credits_enrolled(&self, student_id: &UserId) -> CreditAmount;

    async fn save(&self, enrollment: &Enrollment) -> Result<(), Box<dyn std::error::Error + Send + Sync>>; //matrícula xddd

    async fn completed_courses(&self, student_id: &UserId) -> Result<Vec<CourseId>, String>;

    async fn enrollment_attempts(&self, student_id: &UserId, course_id: &CourseId) -> u8;

    async fn current_enrollments(&self, student_id: &UserId) -> Result<Vec<Enrollment>, String> ;

    async fn find_user_info_by_id(&self, user_id: &UserId) -> Option<(String, Option<String>, String, String)>;
    
    async fn find_any_enrolled_semester(&self, user_id: &UserId) -> Option<String>;
    
    async fn count_enrolled_courses(&self, user_id: &UserId) -> usize;

    async fn find_user_info_by_code(&self, user_code: &UserCode) -> Option<(String, Option<String>, String, String)>;

}
