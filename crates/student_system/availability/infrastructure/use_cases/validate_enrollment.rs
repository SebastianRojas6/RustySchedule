use crate::availability::domain::repository::EnrollmentAvailabilityRepository;
use crate::enrollment::domain::{CourseId, UserId};
use crate::availability::application::validate_enrollment::EnrollmentValidationInput;
use crate::availability::application::validate_enrollment::validate_enrollment;

pub async fn validate_enrollment_use_case<R: EnrollmentAvailabilityRepository>(
    repo: &R,
    student_id: UserId,
    course_id: CourseId,
    section_id: String,
) -> Result<(), anyhow::Error> {
    let course_curriculum = repo.get_course_curriculum(&course_id).await?;
    let student_status = repo.get_student_status(&student_id).await?;
    let student_credits_enrolled = repo.get_student_enrolled_credits(&student_id).await?;
    let course_credits = repo.get_course_credits(&course_id).await?;
    let completed_courses = repo.get_completed_courses(&student_id).await?;
    let course_prerequisites = repo.get_course_prerequisites(&course_id).await?;
    let course_cycle = repo.get_course_cycle(&course_id).await?;
    let section_capacity_available = repo.is_section_capacity_available(&course_id, &section_id).await?;
    let already_enrolled = repo.is_student_already_enrolled(&student_id, &course_id).await?;
    let times_repeated = repo.count_course_repetitions(&student_id, &course_id).await?;
    let schedule_conflict = repo.has_schedule_conflict(&student_id, &course_id, &section_id).await?;
    let course_already_passed = repo.has_already_passed_course(&student_id, &course_id).await?;

    let input = EnrollmentValidationInput {
        student_id,
        course_id,
        course_curriculum,
        student_status,
        student_credits_enrolled,
        course_credits,
        completed_courses,
        course_prerequisites,
        course_cycle,
        section_capacity_available,
        already_enrolled,
        times_repeated,
        schedule_conflict,
        course_already_passed,
    };

    validate_enrollment(input).await
}
