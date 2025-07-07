// crates/student_system/suggestion/infrastructure/fetch_available_data_handler.rs

use actix_web::{web, HttpResponse};
use crate::bootstrap::AppState;
use crate::enrollment::application::find_available_courses_to_enroll::FindAvailableCoursesToEnrollUseCase;
use crate::enrollment::application::find_available_sections_by_course_code::FindAvailableSectionsByCourseCodeUseCase;
use crate::suggestion::application::generate_schedule_suggestion::GenerateScheduleSuggestionUseCase;
use crate::suggestion::infrastructure::repository::SupabaseSuggestionRepository;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct PathStudentId {
    pub student_id: String,
}

pub async fn generate_schedule_suggestion_handler(
    state: web::Data<AppState>,
    path: web::Path<PathStudentId>,
) -> HttpResponse {
    let student_id = path.into_inner().student_id;

    let suggestion_repo = SupabaseSuggestionRepository {
        db: state.enrollment_repo.db.clone(),
    };

    let enrollment_use_case = FindAvailableCoursesToEnrollUseCase::new(state.enrollment_repo.as_ref());
    let section_use_case = FindAvailableSectionsByCourseCodeUseCase::new(state.enrollment_repo.as_ref());
    let suggestion_use_case = GenerateScheduleSuggestionUseCase::new(
        &suggestion_repo,
        &enrollment_use_case,
        &section_use_case,
    );

    match suggestion_use_case.execute(&student_id).await {
        Ok(suggestion) => HttpResponse::Ok().json(suggestion),
        Err(err) => {
            eprintln!("[ERROR] Error en generate_schedule_suggestion_handler: {err:?}");
            HttpResponse::InternalServerError().body(err)
        }
    }
}
