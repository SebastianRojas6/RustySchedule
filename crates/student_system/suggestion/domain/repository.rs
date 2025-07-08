use async_trait::async_trait;

use crate::suggestion::domain::section::TimeSlot;

#[async_trait]
pub trait SuggestionRepository: Send + Sync {
    async fn find_student_status(&self, student_id: &str) -> Option<String>;
    async fn find_availabilities(&self, student_id: &str) -> Vec<TimeSlot>;
}
