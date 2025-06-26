use actix_web::{web, HttpResponse};
use crate::enrollment::domain::*;
use crate::bootstrap::AppState;

pub async fn find_by_id_handler(
    state: web::Data<AppState>,
    id: web::Path<String>,
) -> HttpResponse {
    let id = EnrollmentId::new(id.into_inner());

    match state.enrollment_repo.find_by_id(&id).await {
        Some(enrollment) => HttpResponse::Ok().json(enrollment),
        None => HttpResponse::NotFound().finish(),
    }
}
