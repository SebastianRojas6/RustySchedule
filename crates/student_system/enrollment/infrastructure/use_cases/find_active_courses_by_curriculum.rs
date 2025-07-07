use sea_orm::{DatabaseConnection, EntityTrait, QueryFilter, ColumnTrait};
use crate::enrollment::infrastructure::entity::courses;
use crate::enrollment::domain::available_course::CourseData;

pub async fn find_active_courses_by_curriculum(
    db: &DatabaseConnection,
    semester: &str,
) -> Result<Vec<CourseData>, String> {
    let result = courses::Entity::find()
        .filter(courses::Column::Semester.eq(semester.to_string()))
        .filter(courses::Column::Active.eq(true))
        .all(db)
        .await
        .map_err(|e| e.to_string())?;

    Ok(result
        .into_iter()
        .map(|c| CourseData {
            id: c.id,
            code: c.code,
            name: c.name,
            cycle: c.cycle,
            credits: c.credits,
        })
        .collect())
} 