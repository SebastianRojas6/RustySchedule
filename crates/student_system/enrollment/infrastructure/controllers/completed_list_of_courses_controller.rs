use actix_web::{web, HttpResponse};
use crate::enrollment::domain::*;
use crate::enrollment::infrastructure::use_cases::repository;

pub async fn completed_list_of_courses_handler(
    repo: web::Data<repository::SupabaseEnrollmentRepository>,
    student_id: web::Path<String>,
) -> HttpResponse {
    let student_id = UserId::new(student_id.into_inner());
    let courses = repo.completed_courses(&student_id).await;
    HttpResponse::Ok().json(courses)
}