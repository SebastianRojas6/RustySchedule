use sea_orm::*;
use crate::enrollment::domain::*;
use crate::enrollment::infrastructure::entity::enrollments;
use crate::enrollment::infrastructure::entity::sea_orm_active_enums::EnrollmentStatus as DbEnrollmentStatus;

pub async fn completed_courses(
    db: &DatabaseConnection,
    student_id: &UserId,
) -> Result<Vec<CourseId>, String> {
    let rows = enrollments::Entity::find()
        .filter(
            enrollments::Column::StudentId
                .eq(student_id.value())
                .and(enrollments::Column::Status.eq(DbEnrollmentStatus::Completed)),
        )
        .all(db)
        .await
        .map_err(|e| format!("Database error: {}", e))?;

    Ok(rows.into_iter()
        .map(|r| CourseId::new(r.course_id))
        .collect())
}
