use actix_web::{web, HttpResponse};
use crate::enrollment::domain::*;
use crate::enrollment::infrastructure::use_cases::repository;

pub async fn current_enrollments_handler(
    repo: web::Data<repository::SupabaseEnrollmentRepository>,
    student_id: web::Path<String>,
) -> HttpResponse {
    let student_id = UserId::new(student_id.into_inner());
    let enrollments = repo.current_enrollments(&student_id).await;
    HttpResponse::Ok().json(enrollments)
}
