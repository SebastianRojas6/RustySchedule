use std::sync::Arc;
use crate::availability::domain::repository::EnrollmentAvailabilityRepository;
use crate::availability::application::{
    dto::AvailabilityRequest,
    register_availability::RegisterAvailabilityUseCase,
};

pub struct RegisterAvailabilityService {
    inner: RegisterAvailabilityUseCase,
}

impl RegisterAvailabilityService {
    pub fn new<R: EnrollmentAvailabilityRepository + 'static>(repository: Arc<R>) -> Self {
        Self {
            inner: RegisterAvailabilityUseCase::new(repository),
        }
    }

    pub async fn execute(
        &self,
        student_id: &str,
        request: AvailabilityRequest,
    ) -> Result<(), String> {
        self.inner.execute(student_id, request).await
    }
}
