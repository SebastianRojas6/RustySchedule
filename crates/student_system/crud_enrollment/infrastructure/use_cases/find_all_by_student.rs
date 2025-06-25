use crate::crud_enrollment::domain::crud_enrollment::CrudEnrollment;
use crate::crud_enrollment::domain::repository::CrudEnrollmentRepository;


pub async fn find_all_by_student(
    db: &impl CrudEnrollmentRepository,
    student_id: &str,
) -> Vec<CrudEnrollment> {
    db.find_all_by_student(student_id).await
}