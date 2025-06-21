use crate::enrollment::domain::{Enrollment, EnrollmentStatus, EnrollmentId};
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrudEnrollment {
    pub id: EnrollmentId,
    pub status: EnrollmentStatus,
}

impl CrudEnrollment {
    pub fn from_enrollment(enrollment: &Enrollment) -> Self {
        Self {
            id: enrollment.id.clone(),
            status: enrollment.status.clone(),
        }
    }

    pub fn apply_update(&mut self, new_status: EnrollmentStatus) {
        self.status = new_status;
    }
}
