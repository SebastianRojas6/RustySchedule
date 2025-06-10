use actix_web::{web, HttpResponse};
use crate::enrollment::domain::*;
use crate::enrollment::infrastructure::use_cases::repository;

pub async fn save_handler(
    repo: web::Data<repository::SupabaseEnrollmentRepository>,
    enrollment: web::Json<Enrollment>,
) -> HttpResponse {
    match repo.save(&enrollment).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}
