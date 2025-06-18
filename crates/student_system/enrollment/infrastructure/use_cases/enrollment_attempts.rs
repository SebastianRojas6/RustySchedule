use sea_orm::*;
use crate::enrollment::domain::*;
use crate::enrollment::infrastructure::entity::enrollments;

pub async fn enrollment_attempts(
    db: &DatabaseConnection,
    student_id: &UserId,
    course_id: &CourseId,
) -> u8 {
    let count = enrollments::Entity::find()
        .filter(
            enrollments::Column::StudentId.eq(student_id.value())
                .and(enrollments::Column::CourseId.eq(course_id.value())),
        )
        .count(db)
        .await
        .unwrap_or(0);
    
    count as u8
}
