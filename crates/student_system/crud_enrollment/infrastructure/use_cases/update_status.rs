use sea_orm::{EntityTrait, ActiveModelTrait, Set};
use sea_orm::DbConn;

use crate::enrollment::infrastructure::entity::enrollments;
use crate::enrollment::domain::{EnrollmentId, EnrollmentStatus as DomainStatus};
use crate::enrollment::infrastructure::entity::sea_orm_active_enums::EnrollmentStatus as InfraStatus;

pub async fn update_status(
    db: &DbConn,
    id: &EnrollmentId,
    new_status: DomainStatus,
) -> Result<(), String> {

    let enrollment = enrollments::Entity::find_by_id(id.to_string())
        .one(db)
        .await
        .map_err(|e| format!("DB error: {}", e))?;

    let Some(enrollment) = enrollment else {
        return Err("Enrollment not found".to_string());
    };

    let mut model: enrollments::ActiveModel = enrollment.into();
    model.status = Set(InfraStatus::from(new_status));

    model
        .update(db)
        .await
        .map_err(|e| format!("Update failed: {}", e))?;

    Ok(())
}
