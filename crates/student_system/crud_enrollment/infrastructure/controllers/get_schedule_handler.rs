use actix_web::{web, HttpResponse};
use crate::crud_enrollment::application::get_schedule::GetScheduleService;
use crate::bootstrap::AppState;

pub async fn get_schedule_handler(
    state: web::Data<AppState>,
    path: web::Path<String>,
) -> HttpResponse {
    let student_id = path.into_inner();

    let service = GetScheduleService
    
    {
        repository: &*state.crud_repo,
    };

    match service.execute(&student_id).await {
        Ok(courses) => HttpResponse::Ok().json(courses),
        Err(e) => HttpResponse::InternalServerError().body(e),
    }
}
