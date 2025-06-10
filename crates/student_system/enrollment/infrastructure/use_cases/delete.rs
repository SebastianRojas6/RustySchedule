use sea_orm::*;
use crate::enrollment::domain::*;
use crate::enrollment::infrastructure::entity::enrollments;

pub async fn delete(
    db: &DatabaseConnection,
    id: &EnrollmentId,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    enrollments::Entity::delete_by_id(id.value())
        .exec(db)
        .await?;
    Ok(())
}
