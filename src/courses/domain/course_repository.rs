use super::course::Course;

#[async_trait::async_trait]
pub trait CourseRepository {
    async fn save(&self, course: Course) -> Result<(), String>;
    async fn find_by_name_and_section(&self, name: &str, section: &str) -> Option<Course>;
    async fn list_all(&self) -> Vec<Course>;
}
