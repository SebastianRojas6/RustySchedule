use crate::crud_enrollment::domain::repository::CrudEnrollmentRepository;
use crate::crud_enrollment::domain::enrollment_dto::EnrolledCourseDto;

pub struct GetScheduleService<R: CrudEnrollmentRepository> {
    pub repository: R,
}

impl<R: CrudEnrollmentRepository> GetScheduleService<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    pub async fn execute(&self, student_id: &str) -> Result<Vec<EnrolledCourseDto>, String> {
        self.repository
            .get_schedule(student_id)
            .await
            .map_err(|e| e.to_string())
    }
}
