use sea_orm::*;
use crate::enrollment::domain::*;
use crate::enrollment::infrastructure::entity::enrollments;
use crate::enrollment::domain::EnrollmentStatus as DomainEnrollmentStatus;

pub async fn find_by_student_and_course(
    db: &DatabaseConnection,
    student_id: &UserId,
    course_id: &CourseId,
) -> Option<Enrollment> {
    let result = enrollments::Entity::find()
        .filter(
            enrollments::Column::StudentId.eq(student_id.value())
                .and(enrollments::Column::CourseId.eq(course_id.value())),)
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
