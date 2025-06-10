use actix_web::{web, HttpResponse};
use crate::enrollment::domain::*;
use crate::enrollment::infrastructure::use_cases::repository;

pub async fn find_by_id_handler(
    repo: web::Data<repository::SupabaseEnrollmentRepository>,
    id: web::Path<String>,
) -> HttpResponse {
    let id = EnrollmentId::new(id.into_inner());
    match repo.find_by_id(&id).await {
        Some(enrollment) => HttpResponse::Ok().json(enrollment),
        None => HttpResponse::NotFound().finish(),
    }
}
