use sea_orm::*;
use crate::enrollment::domain::*;
use crate::enrollment::infrastructure::entity::enrollments;
use crate::enrollment::domain::EnrollmentStatus as DomainEnrollmentStatus;

pub async fn find_by_id(
    db: &DatabaseConnection,
    id: &EnrollmentId,
) -> Option<Enrollment> {
    let result = enrollments::Entity::find_by_id(id.value())
        .one(db)
        .await
        .ok()??;

    Some(Enrollment {
        id: EnrollmentId::new(result.id),
        student_id: UserId::new(result.student_id),
        course_id: CourseId::new(result.course_id),
        status: DomainEnrollmentStatus::try_from(result.status).ok()?,
    })
}
