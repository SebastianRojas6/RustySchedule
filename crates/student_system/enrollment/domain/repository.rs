use async_trait::async_trait;
use crate::enrollment::infrastructure::entity::sea_orm_active_enums::UserRole;

use super::super::domain::{Enrollment, EnrollmentId, UserId, CourseId, CreditAmount, UserCode};

use crate::enrollment::domain::available_course::{CourseData, CourseWithSections};

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

    async fn find_user_info_by_code(&self, user_code: &UserCode) -> Option<(String, Option<String>, String, String, UserRole)>;
    
    async fn find_sections_by_course_code(&self, course_code: &str) -> Result<CourseWithSections, String>;

    async fn find_active_courses_by_curriculum(&self, semester: &str) -> Result<Vec<CourseData>, String>;

    async fn find_prerequisites_by_course_id(&self, course_id: &str) -> Result<Vec<CourseId>, String>;

    async fn count_sections_by_course_code(&self, course_code: &str) -> Result<usize, String>;

    async fn find_active_courses_by_semester(&self, semester: &str) -> Result<Vec<CourseData>, String>;

}
