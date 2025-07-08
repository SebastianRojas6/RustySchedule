use sea_orm::{DatabaseConnection, EntityTrait, QueryFilter, ColumnTrait};
use crate::enrollment::infrastructure::entity::course_prerequisites;
use crate::enrollment::domain::CourseId;

pub async fn find_prerequisites_by_course_id(
    db: &DatabaseConnection,
    course_id: &str,
) -> Result<Vec<CourseId>, String> {
    let prereqs = course_prerequisites::Entity::find()
        .filter(course_prerequisites::Column::CourseId.eq(course_id.to_string()))
        .all(db)
        .await
        .map_err(|e| e.to_string())?;

    Ok(prereqs
        .into_iter()
        .map(|r| CourseId(r.prerequisite_course_id))
        .collect())
}
