use sea_orm::DatabaseConnection;
use crate::availability::domain::available_course_dto::AvailableCourseDto;
use crate::availability::infrastructure::use_cases::get_available_courses;
use crate::availability::application::get_all_courses::get_available_courses::get_available_courses_use_case;

pub async fn get_all_courses(
    db: &DatabaseConnection,
    semester: &str,
) -> Vec<AvailableCourseDto> {
    get_available_courses_use_case(db, semester).await
}
