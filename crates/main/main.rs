use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use student_system::bootstrap::init_enrollment_repo;
use student_system::enrollment::infrastructure::actix_web_router::configure_enrollment_routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let repo = init_enrollment_repo().await.expect("Error conectando con Supabase");

    let address = "127.0.0.1:8080";

    println!();
    println!("--> Servidor iniciado con éxito");
    println!("Escuchando en: http://{}", address);

    HttpServer::new(move || App::new().app_data(web::Data::from(repo.clone())).configure(configure_enrollment_routes))
        .bind(address)?
        .run()
        .await
}
