use sea_orm::{DatabaseConnection, EntityTrait, QueryFilter, ColumnTrait, PaginatorTrait};
use crate::enrollment::infrastructure::entity::courses;

pub async fn count_sections_by_course_code(
    db: &DatabaseConnection,
    code: &str,
) -> Result<usize, String> {
    let count = courses::Entity::find()
        .filter(courses::Column::Code.eq(code.to_string()))
        .filter(courses::Column::Active.eq(true))
        .count(db)
        .await
        .map_err(|e| e.to_string())?;

    Ok(count as usize)
}