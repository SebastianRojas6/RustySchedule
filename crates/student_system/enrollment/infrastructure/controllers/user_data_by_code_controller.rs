use actix_web::{web, HttpResponse};
use crate::bootstrap::AppState;
use crate::enrollment::domain::{UserCode, repository::EnrollmentRepository};
use crate::enrollment::infrastructure::entity::sea_orm_active_enums::UserRole;

pub async fn get_user_data_by_code_handler(
    app_state: web::Data<AppState>,
    path: web::Path<String>,
) -> HttpResponse {
    // let student_id = UserId::new(path.into_inner());
    let user_code = UserCode::new(path.into_inner());

    let Some((id, email, specialty, full_name, role)) =
        app_state.enrollment_repo.find_user_info_by_code(&user_code).await
    else {
        return HttpResponse::NotFound().body("User not found");
    };

    // Por si decido recibir mas data tras el login (por ahora solo quiero recibir 200 OK response)
    #[derive(serde::Serialize)]
    struct UserDataResponse {
      id: String,
      email: Option<String>,
      specialty: String,
      full_name: String,
      role: UserRole
    }

    let response = UserDataResponse {
      id,
      email,
      specialty,
      full_name,
      role
    };

    HttpResponse::Ok().json(response)
}
