use actix_web::{App, HttpServer};
use student_system::enrollment::infrastructure::actix_web_router::configure_enrollment_routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Servidor corriendo en http://localhost:8080");

    HttpServer::new(|| {
        App::new()
            .configure(configure_enrollment_routes)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
