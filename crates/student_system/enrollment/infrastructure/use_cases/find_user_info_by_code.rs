use sea_orm::*;
use crate::enrollment::domain::UserCode;
use crate::enrollment::infrastructure::entity::users;

pub async fn find_user_info_by_code(
    db: &DatabaseConnection,
    user_code: &UserCode,
) -> Option<(String, Option<String>, String, String)> {
    let user = users::Entity::find()
        .filter(users::Column::Code.eq(user_code.value()))
        .one(db)
        .await
        .ok()??;

    Some((
        user.id,
        user.email,
        user.specialty,
        user.full_name.unwrap_or_default(),
    ))
}