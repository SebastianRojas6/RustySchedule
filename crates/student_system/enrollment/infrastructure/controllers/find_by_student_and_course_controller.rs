use actix_web::{web, HttpResponse};
use crate::enrollment::domain::*;
use crate::enrollment::infrastructure::use_cases::repository;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct StudentCoursePath {
    pub student_id: String,
    pub course_id: String,
}

pub async fn find_by_student_and_course_handler(
    repo: web::Data<repository::SupabaseEnrollmentRepository>,
    path: web::Path<StudentCoursePath>,
) -> HttpResponse {
    let student_id = UserId::new(path.student_id.clone());
    let course_id = CourseId::new(path.course_id.clone());
    match repo.find_by_student_and_course(&student_id, &course_id).await {
        Some(enrollment) => HttpResponse::Ok().json(enrollment),
        None => HttpResponse::NotFound().finish(),
    }
}