use actix_web::{web, HttpResponse};
use actix_web::web::Query;          
use serde::Deserialize;
use crate::bootstrap::AppState;
use crate::availability::infrastructure::use_cases::get_available_courses::get_available_courses_use_case;

#[derive(Deserialize)]
pub struct AvailableCoursesQuery {
    semester: String,
}

pub async fn get_available_courses_handler(
    state: web::Data<AppState>,
    query: Query<AvailableCoursesQuery>,
) -> HttpResponse {
    let db = &state.enrollment_repo.db;
    let semester = &query.semester;

    match get_available_courses_use_case(db, semester).await {
        courses if courses.is_empty() => HttpResponse::NoContent().finish(),
        courses => HttpResponse::Ok().json(courses),
    }
}