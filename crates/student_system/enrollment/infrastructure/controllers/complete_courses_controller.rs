use actix_web::{web, HttpResponse};
use crate::enrollment::application::complete_course::{CompleteCourseService, CompleteCourse, CompleteCourseError};
use crate::enrollment::domain::EnrollmentId;
use crate::enrollment::infrastructure::use_cases::repository;

pub async fn complete_course_handler(
    repo: web::Data<repository::SupabaseEnrollmentRepository>,
    path: web::Path<String>, // matrícula id
) -> HttpResponse {
    let id = EnrollmentId::new(path.into_inner());

    let service = CompleteCourseService {
        repository: repo.as_ref(),
    };

    match service.complete_course(id).await {
        Ok(()) => HttpResponse::Ok().body("Course completed"),
        Err(CompleteCourseError::NotFound) => HttpResponse::NotFound().body("Enrollment not found"),
        Err(CompleteCourseError::InvalidStatus) => HttpResponse::BadRequest().body("Enrollment not in valid state"),
        Err(e) => {
            eprintln!("Internal error: {:?}", e);
            HttpResponse::InternalServerError().body("Something went wrong")
        }
    }
}
