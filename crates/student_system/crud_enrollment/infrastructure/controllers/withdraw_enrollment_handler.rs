use actix_web::{web, HttpResponse};
use crate::crud_enrollment::application::withdraw::withdraw;
use crate::enrollment::domain::EnrollmentId;
use crate::bootstrap::AppState;

pub async fn withdraw_enrollment_handler(
    repo: web::Data<AppState>,
    path: web::Path<String>,
) -> HttpResponse {
    let enrollment_id = EnrollmentId::new(path.into_inner());

    match withdraw(&*repo.crud_repo, &enrollment_id).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => HttpResponse::InternalServerError().body(format!("Error: {}", e)),
    }
}
