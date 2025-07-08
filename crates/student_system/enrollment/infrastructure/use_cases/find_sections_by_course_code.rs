use sea_orm::{DatabaseConnection, EntityTrait, QueryFilter, ColumnTrait};
use crate::enrollment::infrastructure::entity::{courses, course_schedules};
use crate::enrollment::domain::available_course::{AvailableSection, CourseWithSections};

pub async fn find_sections_by_course_code(
    db: &DatabaseConnection,
    code: &str,
) -> Result<CourseWithSections, String> {
    let sections = courses::Entity::find()
        .filter(courses::Column::Code.eq(code.to_string()))
        .filter(courses::Column::Active.eq(true))
        .all(db)
        .await
        .map_err(|e| e.to_string())?;

    let Some(course) = sections.first() else {
        return Err("No secciones encontradas".into());
    };

    let mut all_sections = vec![];

    for c in &sections {
        let schedule = course_schedules::Entity::find()
            .filter(course_schedules::Column::CourseId.eq(c.id.clone()))
            .all(db)
            .await
            .map_err(|e| e.to_string())?;

        for s in schedule {
            all_sections.push(AvailableSection {
                available_spots: c.available_spots.unwrap_or(0),
                teacher_id: c.teacher_id.clone(),
                day: s.day,
                session_type: s.session_type,
                start_time: s.start_time.format("%H:%M").to_string(),
                end_time: s.end_time.format("%H:%M").to_string(),
            });
        }
    }

    Ok(CourseWithSections {
        nombre_curso: course.name.clone(),
        codigo: course.code.clone(),
        num_ciclo: course.cycle,
        creditos: course.credits,
        secciones: all_sections,
    })
}
