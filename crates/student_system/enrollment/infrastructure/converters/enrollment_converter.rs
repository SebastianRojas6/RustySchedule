use crate::enrollment::domain::{Enrollment, EnrollmentId, UserId, CourseId, EnrollmentStatus as DomainStatus,};
use crate::enrollment::infrastructure::entity::enrollments::Model as EnrollmentModel;
use crate::enrollment::infrastructure::entity::sea_orm_active_enums::EnrollmentStatus as InfraStatus;

impl TryFrom<EnrollmentModel> for Enrollment {
    type Error = ();

    fn try_from(model: EnrollmentModel) -> Result<Self, Self::Error> {
        Ok(Enrollment {
            id: EnrollmentId::new(model.id),
            student_id: UserId::new(model.student_id),
            course_id: CourseId::new(model.course_id),
            status: model.status.into(),
        })
    }
}

impl From<InfraStatus> for DomainStatus {
    fn from(status: InfraStatus) -> Self {
        match status {
            InfraStatus::Enrolled => DomainStatus::Enrolled,
            InfraStatus::Dropped => DomainStatus::Dropped,
            InfraStatus::Completed => DomainStatus::Completed,
            InfraStatus::Failed => DomainStatus::Failed,
            InfraStatus::Pending => DomainStatus::Pending,
            InfraStatus::Withdrawn => DomainStatus::Withdrawn,
        }
    }
}

impl From<DomainStatus> for InfraStatus {
    fn from(status: DomainStatus) -> Self {
        match status {
            DomainStatus::Enrolled => InfraStatus::Enrolled,
            DomainStatus::Dropped => InfraStatus::Dropped,
            DomainStatus::Completed => InfraStatus::Completed,
            DomainStatus::Failed => InfraStatus::Failed,
            DomainStatus::Pending => InfraStatus::Pending,
            DomainStatus::Withdrawn => InfraStatus::Withdrawn,
        }
    }
}
