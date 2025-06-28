use crate::enrollment::domain::{
    CourseCycle, CourseId, CurriculumId, Enrollment, EnrollmentId,
    SemesterParity, StudentStatus, UserId, CreditAmount,
};
use anyhow::Result;

#[derive(Debug)]
pub struct EnrollmentValidationInput {
    pub student_id: UserId,
    pub course_id: CourseId,
    pub course_curriculum: CurriculumId,
    pub student_status: StudentStatus,
    pub student_credits_enrolled: CreditAmount,
    pub course_credits: CreditAmount,
    pub completed_courses: Vec<CourseId>,
    pub course_prerequisites: Vec<CourseId>,
    pub course_cycle: CourseCycle,
    pub section_capacity_available: bool,
    pub already_enrolled: bool,
    pub times_repeated: u8,
    pub schedule_conflict: bool,
    pub course_already_passed: bool,
}

pub async fn validate_enrollment(
    input: EnrollmentValidationInput,
) -> Result<(), anyhow::Error> {

    let semester = match input.course_cycle {
        cycle if cycle.is_even() => SemesterParity::Even,
        _ => SemesterParity::Odd,
    };

    let _enrollment = Enrollment::try_enroll(
        EnrollmentId::generate(),
        input.student_id,
        input.course_id,
        input.student_status,
        input.student_credits_enrolled,
        input.course_credits,
        &input.completed_courses,
        &input.course_prerequisites,
        input.course_cycle,
        semester,
        input.section_capacity_available,
        input.already_enrolled,
        input.times_repeated,
        input.schedule_conflict,
        input.course_already_passed,
    )
    .map_err(anyhow::Error::from)?;

    Ok(())
}

