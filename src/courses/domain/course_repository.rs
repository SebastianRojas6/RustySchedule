use super::course::Course;
use crate::CourseCycle;

#[async_trait::async_trait]
pub trait CourseRepository: Send + Sync {
    async fn save(&self, course: Course) -> Result<(), String>;
    async fn find_by_name_and_section(&self, name: &str, section: &str) -> Option<Course>;
    async fn list_all(&self) -> Vec<Course>;
    async fn find_by_cycle(&self, cycle: CourseCycle) -> Vec<Course>;
}