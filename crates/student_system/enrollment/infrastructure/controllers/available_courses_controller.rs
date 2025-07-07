use actix_web::{web, HttpResponse};
use crate::bootstrap::AppState;
use crate::enrollment::domain::{user_id::UserId, available_course::CourseCode};
use crate::enrollment::application::{
    find_available_courses_to_enroll::FindAvailableCoursesToEnrollUseCase,
    find_available_sections_by_course_code::FindAvailableSectionsByCourseCodeUseCase,
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct PathCourseCode {
    pub code: String,
}

#[derive(Debug, Deserialize)]
pub struct PathStudentId {
    pub student_id: String,
}

pub async fn find_available_courses_to_enroll_handler(
    state: web::Data<AppState>,
    path: web::Path<PathStudentId>,
) -> HttpResponse {
    let repo = state.enrollment_repo.as_ref();
    let use_case = FindAvailableCoursesToEnrollUseCase::new(repo);

    let user_id = UserId::new(path.into_inner().student_id);

    match use_case.execute(&user_id).await {
        Ok(courses) => HttpResponse::Ok().json(courses),
        Err(err) => {
            eprintln!("[ERROR] Error en find_available_courses_to_enroll_handler: {err:?}");
            HttpResponse::InternalServerError().body(err.to_string())
        }
    }
}

pub async fn find_available_sections_by_course_code_handler(
    state: web::Data<AppState>,
    path: web::Path<PathCourseCode>,
) -> HttpResponse {
    let repo = state.enrollment_repo.as_ref();
    let use_case = FindAvailableSectionsByCourseCodeUseCase::new(repo);

    let code = CourseCode(path.into_inner().code);

    match use_case.execute(&code).await {
        Ok(course_sections) => HttpResponse::Ok().json(course_sections),
        Err(err) => {
            eprintln!("[ERROR] Error en find_available_sections_by_course_code_handler: {err:?}");
            HttpResponse::InternalServerError().body(err.to_string())
        }
    }
}
