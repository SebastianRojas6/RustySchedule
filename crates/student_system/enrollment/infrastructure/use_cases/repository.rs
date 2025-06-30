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
    
    async fn find_by_id(&self, id: &EnrollmentId) -> Option<Enrollment> {find_by_id::find_by_id(&self.db, id).await}  

    async fn save(&self, enrollment: &Enrollment) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {save::save(&self.db, enrollment).await}

    async fn completed_courses(&self, student_id: &UserId) -> Result<Vec<CourseId>, String> {completed_courses::completed_courses(&self.db, student_id).await}
    
    async fn current_enrollments(&self, student_id: &UserId) -> Result<Vec<Enrollment>, String> {current_enrollments::current_enrollments(&self.db, student_id).await}

    async fn find_by_student_and_course(&self, student_id: &UserId, course_id: &CourseId) -> Option<Enrollment> {find_by_student_and_course::find_by_student_and_course(&self.db, student_id, course_id).await}

    async fn total_credits_enrolled(&self, student_id: &UserId) -> CreditAmount {total_credits_enrolled::total_credits_enrolled(&self.db, student_id).await}

    async fn enrollment_attempts(&self, student_id: &UserId, course_id: &CourseId) -> u8 {enrollment_attempts::enrollment_attempts(&self.db, student_id, course_id).await}

    async fn find_user_info_by_id(&self, user_id: &UserId, ) -> Option<(String, Option<String>, String, String)> {find_user_info_by_id::find_user_info_by_id(&self.db, user_id).await}

    async fn find_any_enrolled_semester(&self, user_id: &UserId) -> Option<String> { find_any_enrolled_semester::find_any_enrolled_semester(&self.db, user_id).await}

    async fn count_enrolled_courses(&self, user_id: &UserId) -> usize {count_enrolled_courses::count_enrolled_courses(&self.db, user_id).await}

    async fn find_user_info_by_code(&self, user_code: &UserCode) -> Option<(String, Option<String>, String, String)> {find_user_info_by_code::find_user_info_by_code(&self.db, user_code).await}

}
