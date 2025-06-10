use crate::enrollment::domain::enrollment_status::EnrollmentStatus as DomainEnrollmentStatus;
use crate::enrollment::infrastructure::entity::sea_orm_active_enums::EnrollmentStatus as DbEnrollmentStatus;

impl TryFrom<DbEnrollmentStatus> for DomainEnrollmentStatus {
    type Error = ();

    fn try_from(value: DbEnrollmentStatus) -> Result<Self, Self::Error> {
        match value {
            DbEnrollmentStatus::Enrolled => Ok(DomainEnrollmentStatus::Enrolled),
            DbEnrollmentStatus::Withdrawn => Ok(DomainEnrollmentStatus::Dropped),
            DbEnrollmentStatus::Completed => Ok(DomainEnrollmentStatus::Completed),
            _ => Err(()),
        }
    }
}

impl From<DomainEnrollmentStatus> for DbEnrollmentStatus {
    fn from(value: DomainEnrollmentStatus) -> Self {
        match value {
            DomainEnrollmentStatus::Enrolled => DbEnrollmentStatus::Enrolled,
            DomainEnrollmentStatus::Dropped => DbEnrollmentStatus::Withdrawn,
            DomainEnrollmentStatus::Completed => DbEnrollmentStatus::Completed,
        }
    }
}
