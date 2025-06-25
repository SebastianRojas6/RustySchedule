use crate::crud_enrollment::domain::repository::CrudEnrollmentRepository;
use crate::enrollment::domain::{EnrollmentId, EnrollmentStatus};

pub async fn withdraw<R: CrudEnrollmentRepository>(
    repo: &R,
    enrollment_id: &EnrollmentId,
) -> Result<(), String> {
    repo.update_status(enrollment_id, EnrollmentStatus::Withdrawn).await
}
