use actix_web::{web, HttpResponse};
use crate::enrollment::domain::*;
use crate::enrollment::infrastructure::use_cases::repository;

pub async fn delete_handler(
    repo: web::Data<repository::SupabaseEnrollmentRepository>,
    id: web::Path<String>,
) -> HttpResponse {
    let id = EnrollmentId::new(id.into_inner());
    match repo.delete(&id).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}