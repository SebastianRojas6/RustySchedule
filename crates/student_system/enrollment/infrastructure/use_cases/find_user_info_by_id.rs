use sea_orm::*;
use crate::enrollment::domain::UserId;
use crate::enrollment::infrastructure::entity::users;

pub async fn find_user_info_by_id(
    db: &DatabaseConnection,
    user_id: &UserId,
) -> Option<(String, Option<String>, String, String)> {
    let user = users::Entity::find_by_id(user_id.value()) // mejor si tienes `.value()`
        .one(db)
        .await
        .ok()??;

    Some((
        user.code,
        user.email,
        user.specialty,
        user.full_name.unwrap_or_default(), 
    ))
}
