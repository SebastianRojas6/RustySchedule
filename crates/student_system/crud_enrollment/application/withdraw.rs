use crate::crud_enrollment::domain::repository::CrudEnrollmentRepository;
use crate::enrollment::domain::{EnrollmentId, EnrollmentStatus};

pub async fn withdraw<R>(
    repo: &R,
    enrollment_id: &EnrollmentId,
) -> Result<(), String>
where
    for<'a> &'a R: CrudEnrollmentRepository,
{
    repo.update_status(enrollment_id, EnrollmentStatus::Withdrawn).await
}
