use crate::enrollment::domain::{
    Enrollment, EnrollmentError, EnrollmentId, UserId, CourseId,
    CurriculumId, StudentStatus, CreditAmount, CourseCycle,
    SemesterParity, EnrollmentRepository,
};

use async_trait::async_trait;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum EnrollInCourseError {
    #[error("Error en la matrícula: {0}")]
    DomainError(#[from] EnrollmentError),

    #[error("Error en el repositorio: {0}")]
    RepositoryError(#[from] Box<dyn std::error::Error + Send + Sync>),
}

#[async_trait]
pub trait EnrollInCourse {
    async fn enroll(
        &self,
        id: EnrollmentId,
        student_id: UserId,
        course_id: CourseId,
        course_curriculum: CurriculumId,
        student_status: StudentStatus,
        student_credits_enrolled: CreditAmount,
        course_credits: CreditAmount,
        completed_courses: Vec<CourseId>,
        course_prerequisites: Vec<CourseId>,
        course_cycle: CourseCycle,
        semester: SemesterParity,
        section_capacity_available: bool,
        already_enrolled: bool,
        times_repeated: u8,
        schedule_conflict: bool,
        course_already_passed: bool,
    ) -> Result<(), EnrollInCourseError>;
}

pub struct EnrollInCourseService<'a> {
    pub repository: &'a dyn EnrollmentRepository,
}

#[async_trait]
impl<'a> EnrollInCourse for EnrollInCourseService<'a> {
    async fn enroll(
        &self,
        id: EnrollmentId,
        student_id: UserId,
        course_id: CourseId,
        _course_curriculum: CurriculumId,
        student_status: StudentStatus,
        student_credits_enrolled: CreditAmount,
        course_credits: CreditAmount,
        completed_courses: Vec<CourseId>,
        course_prerequisites: Vec<CourseId>,
        course_cycle: CourseCycle,
        semester: SemesterParity,
        section_capacity_available: bool,
        already_enrolled: bool,
        times_repeated: u8,
        schedule_conflict: bool,
        course_already_passed: bool,
    ) -> Result<(), EnrollInCourseError> {
        let enrollment = Enrollment::try_enroll(
            id,
            student_id,
            course_id,
            student_status,
            student_credits_enrolled,
            course_credits,
            &completed_courses,
            &course_prerequisites,
            course_cycle,
            semester,
            section_capacity_available,
            already_enrolled,
            times_repeated,
            schedule_conflict,
            course_already_passed,
        )?;

        self.repository.save(&enrollment).await?;

        Ok(())
    }
}