use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use crate::enrollment::domain::EnrollmentId;
use crate::enrollment::infrastructure::entity::enrollments;
use sea_orm::DatabaseConnection;

pub async fn find_enrollment_id(
    db: &DatabaseConnection,
    student_id: &str,
    course_id: &str,
) -> Option<EnrollmentId> {
    let result = enrollments::Entity::find()
        .filter(enrollments::Column::StudentId.eq(student_id.to_string()))
        .filter(enrollments::Column::CourseId.eq(course_id.to_string()))
        .one(db)
        .await
        .ok()
        .flatten()?;

    Some(EnrollmentId::new(result.id))
}