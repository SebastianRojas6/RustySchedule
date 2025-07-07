use sea_orm::{DatabaseConnection, EntityTrait, QueryFilter, ColumnTrait, PaginatorTrait};

use crate::enrollment::infrastructure::entity::{courses, course_prerequisites};
use crate::enrollment::domain::{
    CourseId,
    available_course::AvailableCourse,
};

pub async fn find_available_courses_to_enroll(
    db: &DatabaseConnection,
    curriculum: &str,
    semester: &str,
    completed_ids: &[CourseId],
    already_enrolled_ids: &[String],
) -> Result<Vec<AvailableCourse>, String> {
    let raw_courses = courses::Entity::find()
        .filter(courses::Column::Curriculum.eq(curriculum))
        .filter(courses::Column::Semester.eq(semester))
        .filter(courses::Column::Active.eq(true))
        .all(db)
        .await
        .map_err(|e| format!("Error al obtener cursos: {}", e))?;

    let mut available_courses = Vec::new();

    for course in raw_courses {
        let prereqs = course_prerequisites::Entity::find()
            .filter(course_prerequisites::Column::CourseId.eq(&course.id))
            .all(db)
            .await
            .map_err(|e| format!("Error al obtener prerrequisitos del curso {}: {}", course.id, e))?;

        let all_prereqs_completed = prereqs.iter().all(|p| {
            completed_ids.contains(&CourseId(p.prerequisite_course_id.clone()))
        });

        let course_id = CourseId(course.id.clone());

        let not_completed = !completed_ids.contains(&course_id);
        let not_enrolled = !already_enrolled_ids.contains(&course.id);

        if all_prereqs_completed && not_completed && not_enrolled {
            let section_count = courses::Entity::find()
                .filter(courses::Column::Code.eq(&course.code))
                .filter(courses::Column::Active.eq(true))
                .count(db)
                .await
                .map_err(|e| format!("Error al contar secciones para curso {}: {}", course.code, e))?;

            available_courses.push(AvailableCourse {
                code: course.code,
                name: course.name,
                cycle: course.cycle,
                credits: course.credits,
                num_sections: section_count as usize,
            });
        }
    }

    Ok(available_courses)
}
