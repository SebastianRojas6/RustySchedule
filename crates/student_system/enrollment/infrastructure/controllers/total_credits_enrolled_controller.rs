use actix_web::{web, HttpResponse};
use crate::enrollment::domain::*;
use crate::enrollment::infrastructure::use_cases::repository;

pub async fn total_credits_enrolled_handler(
    repo: web::Data<repository::SupabaseEnrollmentRepository>,
    student_id: web::Path<String>,
) -> HttpResponse {
    let student_id = UserId::new(student_id.into_inner());
    let credits = repo.total_credits_enrolled(&student_id).await;
    HttpResponse::Ok().json(credits)

}