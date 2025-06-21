use crate::crud_enrollment::domain::repository::CrudEnrollmentRepository;
use crate::enrollment::domain::{EnrollmentId, EnrollmentStatus};

pub async fn update_status<R: CrudEnrollmentRepository>(
    repo: &R,
    id: &EnrollmentId,
    new_status: EnrollmentStatus,
) -> Result<(), String> {
    repo.update_status(id, new_status).await
}
