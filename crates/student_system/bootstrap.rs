use std::sync::Arc;

use shared::config::connect_to_supabase;
use crate::enrollment::infrastructure::use_cases::repository::SupabaseEnrollmentRepository;

pub async fn init_enrollment_repo() -> Result<Arc<SupabaseEnrollmentRepository>, Box<dyn std::error::Error + Send + Sync>> {
    let db = connect_to_supabase().await?;
    Ok(Arc::new(SupabaseEnrollmentRepository { db }))
}
