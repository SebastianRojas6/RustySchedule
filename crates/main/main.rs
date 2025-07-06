use actix_cors::Cors;
use actix_web::middleware::Logger;
use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use student_system::actix_web_router::configure_enrollment_routes;
use student_system::bootstrap::init_state;
use teacher_system::infrastructure::api_restful::config::boostrap::bootstrap_teacher;
use teacher_system::infrastructure::api_restful::routes::configure_teacher_routes;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let app_student = init_state().await.expect("Failed to initialize application state");

    let app_teacher = bootstrap_teacher().await.expect("Failed to initialize application state");

    let shared_data_student = web::Data::new(app_student);
    let shared_data_teacher = web::Data::new(app_teacher);

    println!("Server running on http://localhost:8080");

    HttpServer::new(move || {
        App::new()
            .wrap(Cors::permissive())
            .wrap(Logger::default())
            .app_data(shared_data_student.clone())
            .app_data(shared_data_teacher.clone())
            .configure(configure_teacher_routes)
            .configure(configure_enrollment_routes)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
