use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, QueryOrder};
use crate::enrollment::infrastructure::entity::courses;
use crate::availability::domain::available_course_dto::AvailableCourseDto;

pub async fn get_available_courses_use_case(
    db: &DatabaseConnection,
    semester: &str,
) -> Vec<AvailableCourseDto> {
    let course_models = courses::Entity::find()
        .filter(courses::Column::Active.eq(true))
        .filter(courses::Column::Semester.eq(semester))
        .filter(courses::Column::AvailableSpots.gt(0))
        .order_by_asc(courses::Column::Cycle)
        .all(db)
        .await
        .unwrap_or_default(); 

    course_models
        .into_iter()
        .map(|course| AvailableCourseDto {
            id: course.id,
            code: course.code,
            name: course.name,
            credits: course.credits,
            cycle: course.cycle,
            teacher_id: course.teacher_id,
            section: course.section,
            curriculum: course.curriculum.to_string(),
        })
        .collect()
}
