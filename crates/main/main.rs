use actix_web::{App, HttpServer};
use dotenv::dotenv;
use student_system::enrollment::infrastructure::actix_web_router::configure_enrollment_routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let server = HttpServer::new(|| App::new().configure(configure_enrollment_routes)).bind(("127.0.0.1", 8080))?;

    println!("Servidor corriendo en http://localhost:8080");

    server.run().await
}
