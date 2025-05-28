use crate::domain::{course::Course, course_repository::CourseRepository};

pub struct CourseService<'a> {
    pub repo: &'a dyn CourseRepository,
}

impl<'a> CourseService<'a> {
    pub async fn get_all(&self) -> Vec<Course> {
        self.repo.list_all().await
    }

    pub async fn get_by_cycle(&self, cycle: i32) -> Vec<Course> {
        self.repo.find_by_cycle(cycle).await
    }
}
