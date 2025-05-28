use crate::courses::domain::course::Course;
use crate::courses::domain::course_repository::CourseRepository;

pub struct GetAllCoursesUseCase {
    repository: Box<dyn CourseRepository + Send + Sync>,
}

impl GetAllCoursesUseCase {
    pub fn new(repository: Box<dyn CourseRepository + Send + Sync>) -> Self {
        Self { repository }
    }

    pub async fn execute(&self) -> Vec<Course> {
        self.repository.list_all().await
    }
}
