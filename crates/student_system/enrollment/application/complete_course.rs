use crate::enrollment::domain::{EnrollmentId, EnrollmentRepository};
use async_trait::async_trait;
use std::fmt::Debug;
use crate::enrollment::domain::EnrollmentError;

#[derive(Debug, thiserror::Error)]
pub enum CompleteCourseError {
    #[error("Enrollment not found")]
    NotFound,

    #[error(transparent)]
    DomainError(#[from] EnrollmentError),

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

        enrollment.complete()?;

    self.repository.save(&enrollment).await?;

    Ok(())
}}
