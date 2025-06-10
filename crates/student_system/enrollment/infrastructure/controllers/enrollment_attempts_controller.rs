/*use actix_web::{web, HttpResponse};
use crate::enrollment::infrastructure::use_cases::repository::SupabaseEnrollmentRepository;
use crate::enrollment::domain::{UserId, CourseId};

pub async fn enrollment_attempts_handler(
    path: web::Path<(String, String)>,
) -> HttpResponse {
    let (student_id_str, course_id_str) = path.into_inner();

    let student_id = UserId::new(student_id_str);
    let course_id = CourseId::new(course_id_str);

    let repository = match SupabaseEnrollmentRepository::new().await {
        Ok(repo) => repo,
        Err(err) => {
            eprintln!("[ERROR] Error creando repositorio: {:?}", err);
            return HttpResponse::InternalServerError().finish();
        }
    };

    let attempts = repository.enrollment_attempts(&student_id, &course_id).await;

    HttpResponse::Ok().json(attempts)
}*/