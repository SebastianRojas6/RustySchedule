use sea_orm::{DatabaseConnection, EntityTrait, ColumnTrait, QueryFilter};

use crate::enrollment::infrastructure::entity::courses;
use crate::enrollment::domain::available_course::CourseData;

pub async fn find_active_courses_by_semester(
    db: &DatabaseConnection,
    semester: &str,
) -> Result<Vec<CourseData>, String> {
    let raw_courses = courses::Entity::find()
        .filter(courses::Column::Semester.eq(semester))
        .filter(courses::Column::Active.eq(true))
        .all(db)
        .await
        .map_err(|e| format!("Error al obtener cursos activos: {e}"))?;

    Ok(raw_courses
        .into_iter()
        .map(|c| CourseData {
            id: c.id,
            code: c.code,
            name: c.name,
            credits: c.credits,
            cycle: c.cycle,
        })
        .collect())
}
