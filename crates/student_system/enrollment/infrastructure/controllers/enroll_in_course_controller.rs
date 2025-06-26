use actix_web::{web, HttpResponse};
use crate::enrollment::application::enroll_in_course::{EnrollInCourseService, EnrollInCourse};
use crate::enrollment::domain::*;
use crate::enrollment::infrastructure::controllers::enroll_in_course_dto::EnrollInCourseRequestDto;
use crate::bootstrap::AppState;
use std::convert::TryFrom;

pub async fn enroll_in_course_handler(
    state: web::Data<AppState>,
    request: web::Json<EnrollInCourseRequestDto>,
) -> HttpResponse {
    let dto = request.into_inner();

    let student_status = match StudentStatus::try_from(dto.student_status.as_str()) {
        Ok(status) => status,
        Err(_) => return HttpResponse::BadRequest().body("Estado de estudiante inválido"),
    };

    let semester_parity = match SemesterParity::try_from(dto.semester.as_str()) {
        Ok(s) => s,
        Err(_) => return HttpResponse::BadRequest().body("Semestre inválido"),
    };

    let service = EnrollInCourseService {
        repository: state.enrollment_repo.as_ref() as &dyn EnrollmentRepository,
    };

    let result = service
        .enroll(
            EnrollmentId::generate(),
            UserId::new(dto.student_id),
            CourseId::new(dto.course_id),
            CurriculumId::new(dto.student_curriculum),
            CurriculumId::new(dto.course_curriculum),
            student_status,
            CreditAmount::new(dto.student_credits_enrolled as u8),
            CreditAmount::new(dto.course_credits as u8),
            dto.completed_courses.into_iter().map(CourseId::new).collect(),
            dto.course_prerequisites.into_iter().map(CourseId::new).collect(),
            CourseCycle::new(dto.course_cycle),
            semester_parity,
            dto.section_capacity_available,
            dto.already_enrolled,
            dto.times_repeated,
            dto.schedule_conflict,
            dto.course_already_passed,
        )
        .await;

    match result {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(err) => {
            eprintln!("[ERROR] Error en enroll_in_course_handler: {:?}", err);
            HttpResponse::BadRequest().body(err.to_string())
        }
    }
}
