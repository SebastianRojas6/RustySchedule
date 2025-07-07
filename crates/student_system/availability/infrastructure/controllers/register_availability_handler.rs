use actix_web::{web, HttpResponse};
use crate::bootstrap::AppState;
use crate::availability::application::dto::AvailabilityRequest;
use crate::availability::infrastructure::use_cases::register_availability::RegisterAvailabilityService;

pub async fn register_availability_handler(
    state: web::Data<AppState>,
    path: web::Path<String>,
    request: web::Json<AvailabilityRequest>,
) -> HttpResponse {
    let student_id = path.into_inner();
    let req = request.into_inner();

    let service = RegisterAvailabilityService::new(state.availability_repo.clone());

    match service.execute(&student_id, req).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => HttpResponse::BadRequest().body(e),
    }
}
