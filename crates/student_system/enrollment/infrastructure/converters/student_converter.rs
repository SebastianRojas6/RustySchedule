use crate::enrollment::domain::student_status::StudentStatus as DomainStudentStatus;
use crate::enrollment::infrastructure::entity::sea_orm_active_enums::StudentStatus as DbStudentStatus;

impl From<DbStudentStatus> for DomainStudentStatus {
    fn from(status: DbStudentStatus) -> Self {
        match status {
            DbStudentStatus::Regular => DomainStudentStatus::Regular,
            DbStudentStatus::Observado => DomainStudentStatus::Observed,
            DbStudentStatus::Graduated => DomainStudentStatus::Graduated,
        }
    }
}
