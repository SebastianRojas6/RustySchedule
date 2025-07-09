use sea_orm::{Database, DatabaseConnection, ConnectOptions};
use std::env;
use std::time::Duration;

pub async fn connect_to_supabase() -> Result<DatabaseConnection, Box<dyn std::error::Error + Send + Sync>> {
    let database_url = env::var("SUPABASE_URL_2")?;

    let mut options = ConnectOptions::new(database_url);
    options
        .max_connections(30) 
        .connect_timeout(Duration::from_secs(5))
        .acquire_timeout(Duration::from_secs(5))
        .idle_timeout(Duration::from_secs(60))
        .sqlx_logging(false);

    let db = Database::connect(options).await?;
    Ok(db)
}
