use actix_web::{App, HttpServer, web};
use actix_cors::Cors;
use student_system::bootstrap::init_state;
use student_system::actix_web_router::configure_enrollment_routes;
use dotenv::dotenv;

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    dotenv().ok();

    let app_state = init_state()
        .await
        .expect("Failed to initialize application state");

    let shared_data = web::Data::new(app_state);

    HttpServer::new(move || {
        App::new()
            .wrap(Cors::permissive())
            .app_data(shared_data.clone())
            .configure(configure_enrollment_routes)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}