use actix_web::{web, HttpResponse};
use crate::bootstrap::AppState;
use crate::availability::domain::validate_enrollment_dto::ValidateEnrollmentDto;
use crate::availability::infrastructure::use_cases::validate_enrollment::validate_enrollment_use_case;
use crate::enrollment::domain::{UserId, CourseId};

pub async fn validate_enrollment_handler(
    state: web::Data<AppState>,
    payload: web::Json<ValidateEnrollmentDto>,
) -> HttpResponse {
    let repo = &state.availability_repo;

    let student_id = UserId::new(payload.student_id.clone());
    let course_id = CourseId::new(payload.course_id.clone());
    let section_id = payload.section_id.clone();

    match validate_enrollment_use_case(repo.as_ref(), student_id, course_id, section_id).await {
        Ok(_) => HttpResponse::Ok().body("Valid enrollment"),
        Err(err) => HttpResponse::BadRequest().body(format!("Validation failed: {}", err)),
    }
}
