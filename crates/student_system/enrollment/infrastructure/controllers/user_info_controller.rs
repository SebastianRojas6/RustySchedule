use actix_web::{web, HttpResponse};
use crate::bootstrap::AppState;
use crate::enrollment::domain::{UserId, repository::EnrollmentRepository};
use crate::enrollment::domain::user_info::UserInfoResponse;

pub async fn get_user_info_handler(
    app_state: web::Data<AppState>,
    path: web::Path<String>,
) -> HttpResponse {
    let student_id = UserId::new(path.into_inner());

    let Some((code, email, specialty, full_name)) =
        app_state.enrollment_repo.find_user_info_by_id(&student_id).await
    else {
        return HttpResponse::NotFound().body("User not found");
    };

    let semester = app_state
        .enrollment_repo
        .find_any_enrolled_semester(&student_id)
        .await
        .unwrap_or_else(|| "Sin matrícula activa".to_string());

    let cursos_matriculados = app_state
        .enrollment_repo
        .count_enrolled_courses(&student_id)
        .await;

    let creditos_totales = app_state
        .enrollment_repo
        .total_credits_enrolled(&student_id)
        .await
        .value()
        .into();

    let response = UserInfoResponse {
        code,
        email,
        specialty,
        full_name,
        semester,
        cursos_matriculados,
        creditos_totales,
    };

    HttpResponse::Ok().json(response)
}
