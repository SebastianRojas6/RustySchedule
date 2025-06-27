use actix_web::web::{self, ServiceConfig};
use crate::enrollment::infrastructure::controllers::{
    find_by_id_handler,
    find_by_student_and_course_handler,
    current_enrollments_handler,
    completed_list_of_courses_handler,
    total_credits_enrolled_handler,
    complete_course_handler,
    enroll_in_course_handler
};

use crate::crud_enrollment::infrastructure::controllers::{
    get_schedule_handler,
    withdraw_enrollment_handler,
};


pub fn configure_enrollment_routes(cfg: &mut ServiceConfig) {
    cfg.service(
        web::scope("/enrollments")
        
            .route("/{id}", web::get().to(find_by_id_handler))
            .route("/student/{student_id}/course/{course_id}", web::get().to(find_by_student_and_course_handler))
            .route("/student/{student_id}/current", web::get().to(current_enrollments_handler))
            .route("/student/{student_id}/completed", web::get().to(completed_list_of_courses_handler))
            .route("/student/{student_id}/credits", web::get().to(total_credits_enrolled_handler))
            .route("/{id}/complete", web::post().to(complete_course_handler))
            .route("/student/course/enroll", web::post().to(enroll_in_course_handler))
            
            .route("/student/{student_id}/enrollments", web::get().to(get_schedule_handler))
            .route("/{id}/withdraw", web::post().to(withdraw_enrollment_handler))
    );
}
