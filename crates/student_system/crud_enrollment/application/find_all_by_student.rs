use crate::crud_enrollment::domain::repository::CrudEnrollmentRepository;
use crate::crud_enrollment::domain::crud_enrollment::CrudEnrollment;

pub async fn find_all_by_student<R: CrudEnrollmentRepository>(
    repo: &R,
    student_id: &str,
) -> Vec<CrudEnrollment> {
    repo.find_all_by_student(student_id).await
}
