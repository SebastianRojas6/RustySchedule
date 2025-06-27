use actix_web::{web, HttpResponse};
use crate::enrollment::domain::*;
use crate::bootstrap::AppState;
use crate::enrollment::domain::repository::EnrollmentRepository; // Importa el trait

pub async fn total_credits_enrolled_handler(
    app_state: web::Data<AppState>,
    student_id: web::Path<String>,
) -> HttpResponse {
    let student_id = UserId::new(student_id.into_inner());

    let credits = app_state.enrollment_repo.total_credits_enrolled(&student_id).await;

    HttpResponse::Ok().json(credits)
}
