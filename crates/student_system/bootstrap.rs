use std::sync::Arc;

use crate::enrollment::infrastructure::use_cases::repository::SupabaseEnrollmentRepository;
use crate::crud_enrollment::infrastructure::use_cases::repository::SupabaseCrudEnrollmentRepository;
use crate::availability::infrastructure::use_cases::repository::SupabaseAvailabilityRepository;
use crate::suggestion::infrastructure::repository::SupabaseSuggestionRepository;
use shared::config::connect_to_supabase;

pub struct AppState {
    pub crud_repo: Arc<SupabaseCrudEnrollmentRepository>,
    pub enrollment_repo: Arc<SupabaseEnrollmentRepository>,
    pub availability_repo: Arc<SupabaseAvailabilityRepository>,
    pub suggestion_repo: Arc<SupabaseSuggestionRepository>,
}

pub async fn init_state() -> Result<AppState, Box<dyn std::error::Error + Send + Sync>> {
    let db = connect_to_supabase().await?;

    Ok(AppState {
        crud_repo: Arc::new(SupabaseCrudEnrollmentRepository::new(db.clone())),
        enrollment_repo: Arc::new(SupabaseEnrollmentRepository { db: db.clone() }),
        availability_repo: Arc::new(SupabaseAvailabilityRepository { db : db.clone()}),
        suggestion_repo: Arc::new(SupabaseSuggestionRepository { db }),
    })
}
