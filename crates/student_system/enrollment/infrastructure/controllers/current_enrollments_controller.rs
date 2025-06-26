use actix_web::{web, HttpResponse};
use crate::enrollment::domain::*;
use crate::bootstrap::AppState;

pub async fn current_enrollments_handler(
    state: web::Data<AppState>,
    student_id: web::Path<String>,
) -> HttpResponse {
    let student_id = UserId::new(student_id.into_inner());

    match state.enrollment_repo.current_enrollments(&student_id).await {
        Ok(enrollments) => HttpResponse::Ok().json(enrollments),
        Err(err) => HttpResponse::InternalServerError().body(format!("Error: {}", err)),
    }
}
