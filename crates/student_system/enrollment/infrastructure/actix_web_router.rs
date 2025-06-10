use actix_web::web::{self, ServiceConfig};
use crate::enrollment::infrastructure::controllers::{
    save_handler,
    find_by_id_handler,
    find_by_student_and_course_handler,
    delete_handler,
    current_enrollments_handler,
    completed_list_of_courses_handler,
    total_credits_enrolled_handler,
    complete_course_handler
};

pub fn configure_enrollment_routes(cfg: &mut ServiceConfig) {
    cfg.service(
        web::scope("/enrollments")
            .route("/save", web::post().to(save_handler))
            .route("/{id}", web::get().to(find_by_id_handler))
            .route("/student/{student_id}/course/{course_id}", web::get().to(find_by_student_and_course_handler))
            .route("/{id}/delete", web::delete().to(delete_handler))
            .route("/student/{student_id}/current", web::get().to(current_enrollments_handler))
            .route("/student/{student_id}/completed", web::get().to(completed_list_of_courses_handler))
            .route("/student/{student_id}/credits", web::get().to(total_credits_enrolled_handler))
            .route("/{id}/complete", web::post().to(complete_course_handler))
    );
}
