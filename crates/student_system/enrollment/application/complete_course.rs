use crate::enrollment::domain::{EnrollmentId, EnrollmentRepository, EnrollmentStatus,};
use async_trait::async_trait;
use std::fmt::Debug;

#[derive(Debug, thiserror::Error)]
pub enum CompleteCourseError {
    #[error("Enrollment not found")]
    NotFound,

    #[error("Enrollment is not in an Enrolled state")]
    InvalidStatus,

    #[error("Repository error: {0}")]
    RepositoryError(#[from] Box<dyn std::error::Error + Send + Sync>),
}

#[async_trait]
pub trait CompleteCourse {
    async fn complete_course(&self, id: EnrollmentId) -> Result<(), CompleteCourseError>;
}

pub struct CompleteCourseService<'a> {
    pub repository: &'a dyn EnrollmentRepository,
}

#[async_trait]
impl<'a> CompleteCourse for CompleteCourseService<'a> {
    async fn complete_course(&self, id: EnrollmentId) -> Result<(), CompleteCourseError> {
        let mut enrollment = self
            .repository
            .find_by_id(&id)
            .await
            .ok_or(CompleteCourseError::NotFound)?;

        if enrollment.status != EnrollmentStatus::Enrolled {
            return Err(CompleteCourseError::InvalidStatus);
        }

        enrollment.complete();

        self.repository.save(&enrollment).await?;

        Ok(())
    }
}
