use actix_web::web::{self, ServiceConfig};

use crate::enrollment::infrastructure::controllers::{
    find_by_id_handler,
    find_by_student_and_course_handler,
    current_enrollments_handler,
    completed_list_of_courses_handler,
    total_credits_enrolled_handler,
    complete_course_handler,
    enroll_in_course_handler,
    get_user_info_handler,
    get_user_data_by_code_handler,
    find_available_courses_to_enroll_handler,
    find_available_sections_by_course_code_handler,
};

use crate::crud_enrollment::infrastructure::controllers::{
    get_schedule_handler,
    withdraw_enrollment_handler,
};

use crate::availability::infrastructure::controllers::{
    get_available_courses_handler,
    validate_enrollment_handler,
    register_availability_handler,
};

pub fn configure_enrollment_routes(cfg: &mut ServiceConfig) {
    cfg.service(
        web::scope("/student")
        
            .route("/{id}", web::get().to(find_by_id_handler))
            .route("/{student_id}/course/{course_id}", web::get().to(find_by_student_and_course_handler))
            .route("/{student_id}/current", web::get().to(current_enrollments_handler))
            .route("/{student_id}/completed", web::get().to(completed_list_of_courses_handler))
            .route("/{student_id}/credits", web::get().to(total_credits_enrolled_handler))
            .route("/{id}/complete", web::post().to(complete_course_handler))
            .route("/course/enroll", web::post().to(enroll_in_course_handler))
            .route("/{student_id}/info", web::get().to(get_user_info_handler))
            .route("/code/{user_code}/info", web::get().to(get_user_data_by_code_handler))

            //Nuevo
            .route("/{student_id}/available-courses", web::get().to(find_available_courses_to_enroll_handler))
            .route("/available-sections/{code}", web::get().to(find_available_sections_by_course_code_handler))

            .route("/{student_id}/enrollments", web::get().to(get_schedule_handler))
            .route("/{id}/withdraw", web::post().to(withdraw_enrollment_handler))

            .route("/available-courses", web::get().to(get_available_courses_handler))
            .route("/validate", web::post().to(validate_enrollment_handler))
            .route("/{student_id}/availability", web::post().to(register_availability_handler))
    );
}
