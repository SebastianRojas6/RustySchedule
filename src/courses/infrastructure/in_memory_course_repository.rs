use std::sync::{Arc, RwLock};

use async_trait::async_trait;

use crate::courses::domain::course::Course;
use crate::courses::domain::course_repository::CourseRepository;
use crate::CourseCycle;

pub struct InMemoryCourseRepository {
    courses: Arc<RwLock<Vec<Course>>>,
}

impl InMemoryCourseRepository {
    pub fn new() -> Self {
        Self {
            courses: Arc::new(RwLock::new(vec![])),}
    }
}

#[async_trait]
impl CourseRepository for InMemoryCourseRepository {
    async fn save(&self, course: Course) -> Result<(), String> {
        let mut courses = self.courses.write().unwrap();

        let exists = courses.iter().any(|c| {
            c.name == course.name && c.section == course.section
        });

        if exists {
            return Err("Ya existe un curso con ese nombre y sección".to_string());
        }

        courses.push(course);
        Ok(())
    }

    async fn find_by_name_and_section(&self, name: &str, section: &str) -> Option<Course> {
        let courses = self.courses.read().unwrap();
        courses
            .iter()
            .find(|c| c.name.value() == name && c.section.value().to_string() == section)
            .cloned()
    }

    async fn list_all(&self) -> Vec<Course> {
        let courses = self.courses.read().unwrap();
        courses.clone()
    }

    async fn find_by_cycle(&self, cycle: CourseCycle) -> Vec<Course> {
        let courses = self.courses.read().unwrap();
        courses
            .iter()
            .filter(|c| c.cycle == cycle)
            .cloned()
            .collect()
    }
}
