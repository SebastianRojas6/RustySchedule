use async_trait::async_trait;
use crate::enrollment::domain::{
    CourseId, CurriculumId, CreditAmount, CourseCycle, SemesterParity, StudentStatus, UserId,
};

#[async_trait]
pub trait EnrollmentAvailabilityRepository: Send + Sync {
    async fn get_student_curriculum(&self, student_id: &UserId) -> anyhow::Result<CurriculumId>;
    async fn get_course_curriculum(&self, course_id: &CourseId) -> anyhow::Result<CurriculumId>;
    async fn get_student_status(&self, student_id: &UserId) -> anyhow::Result<StudentStatus>;
    async fn get_student_enrolled_credits(&self, student_id: &UserId) -> anyhow::Result<CreditAmount>;
    async fn get_course_credits(&self, course_id: &CourseId) -> anyhow::Result<CreditAmount>;
    async fn get_completed_courses(&self, student_id: &UserId) -> anyhow::Result<Vec<CourseId>>;
    async fn get_course_prerequisites(&self, course_id: &CourseId) -> anyhow::Result<Vec<CourseId>>;
    async fn get_course_cycle(&self, course_id: &CourseId) -> anyhow::Result<CourseCycle>;
    async fn get_course_semester_parity(&self, course_id: &CourseId) -> anyhow::Result<SemesterParity>;
    async fn is_section_capacity_available(&self, course_id: &CourseId, section_id: &str) -> anyhow::Result<bool>;
    async fn is_student_already_enrolled(&self, student_id: &UserId, course_id: &CourseId) -> anyhow::Result<bool>;
    async fn count_course_repetitions(&self, student_id: &UserId, course_id: &CourseId) -> anyhow::Result<u8>;
    async fn has_schedule_conflict(&self, student_id: &UserId, course_id: &CourseId, section_id: &str) -> anyhow::Result<bool>;
    async fn has_already_passed_course(&self, student_id: &UserId, course_id: &CourseId) -> anyhow::Result<bool>;
}
