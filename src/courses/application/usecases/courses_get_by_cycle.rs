use crate::courses::domain::course::{Course, CourseCycle};
use crate::courses::domain::course_repository::CourseRepository;

/*use crate::courses::domain::course_schedule::{
    Day,
    SessionType,};*/

    pub struct GetCoursesByCycleUseCase<R: CourseRepository> {
    repository: R,
}

impl<R: CourseRepository> GetCoursesByCycleUseCase<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    pub async fn execute(&self, cycle: CourseCycle) -> Vec<Course> {
        self.repository.find_by_cycle(cycle).await
    }
}

/* Test

#[cfg(test)]
mod tests {
    use super::*;
    use async_trait::async_trait;
    use crate::courses::domain::course::*;
    use crate::courses::domain::course_repository::CourseRepository;

    struct MockCourseRepository {
        courses: Vec<Course>,
    }

    impl MockCourseRepository {
        fn new(courses: Vec<Course>) -> Self {
            Self { courses }
        }
    }

    #[async_trait]
    impl CourseRepository for MockCourseRepository {
        async fn save(&self, _course: Course) -> Result<(), String> {
            unimplemented!()
        }

        async fn find_by_name_and_section(&self, _name: &str, _section: &str) -> Option<Course> {
            unimplemented!()
        }

        async fn list_all(&self) -> Vec<Course> {
            self.courses.clone()
        }

        async fn find_by_cycle(&self, cycle: CourseCycle) -> Vec<Course> {
            self.courses
                .iter()
                .cloned()
                .filter(|c| c.cycle == cycle)
                .collect()
        }
    }

    #[tokio::test]
    async fn test_get_courses_by_cycle_returns_correct_courses() {
        let cycle = CourseCycle::new(3).unwrap();

        let course1 = Course {
            code: CourseCode::new("MAT101".into()).unwrap(),
            name: CourseName::new("Cálculo I".into()).unwrap(),
            section: CourseSection::new(1).unwrap(),
            curriculum: CourseCurriculum::M2023,
            capacity: CourseCapacity::new(45).unwrap(),
            credits: CourseCredits::new(4).unwrap(),
            prerequisites: vec!["math001".into()],
            teacher_id: "teacher-123".into(),
            facility_id: "room-01".into(),
            cycle,
            enrolled: 0, // valor inicial de inscritos
            schedule: vec![
            Schedule {
            day: Day::Lunes,
            start_time: "08:00".into(),
            end_time: "10:00".into(),
            room: "Aula 101".into(),
            session_type: SessionType::Teoria,
            },
            Schedule {
            day: Day::Miércoles,
            start_time: "08:00".into(),
            end_time: "10:00".into(),
            room: "Aula 101".into(),
            session_type: SessionType::Teoria,
        },
    ],
        };

        let course2 = Course {
            cycle: CourseCycle::new(1).unwrap(),
            ..course1.clone()
        };

        let mock_repo = MockCourseRepository::new(vec![course1.clone(), course2]);
        let use_case = GetCoursesByCycleUseCase::new(mock_repo);

        let result = use_case.execute(cycle).await;
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].code.value(), "MAT101");
    }
}

*/