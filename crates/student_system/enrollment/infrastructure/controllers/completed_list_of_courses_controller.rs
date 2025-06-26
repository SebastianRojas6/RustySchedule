use actix_web::{web, HttpResponse};
use crate::enrollment::domain::*;
use crate::bootstrap::AppState;

pub async fn completed_list_of_courses_handler(
    state: web::Data<AppState>,
    student_id: web::Path<String>,
) -> HttpResponse {
    let student_id = UserId::new(student_id.into_inner());
    let repo = &state.enrollment_repo;

    match repo.completed_courses(&student_id).await {
        Ok(courses) => HttpResponse::Ok().json(courses),
        Err(err) => HttpResponse::InternalServerError().body(format!("Error: {}", err)),
    }
}