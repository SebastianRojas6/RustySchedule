use actix_web::{web, HttpResponse};
use crate::enrollment::domain::{UserId, CourseId};
use crate::bootstrap::AppState;
use crate::enrollment::domain::repository::EnrollmentRepository;

pub async fn enrollment_attempts_handler(
    state: web::Data<AppState>,
    path: web::Path<(String, String)>,
) -> HttpResponse {
    let (student_id_str, course_id_str) = path.into_inner();

    let student_id = UserId::new(student_id_str);
    let course_id = CourseId::new(course_id_str);

    let attempts = state
        .enrollment_repo
        .enrollment_attempts(&student_id, &course_id)
        .await;

    HttpResponse::Ok().json(attempts)
}
