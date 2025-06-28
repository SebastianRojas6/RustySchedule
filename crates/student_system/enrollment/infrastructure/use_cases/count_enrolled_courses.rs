use sea_orm::*;
use crate::enrollment::domain::UserId;
use crate::enrollment::infrastructure::entity::enrollments;
use std::convert::TryInto;

pub async fn count_enrolled_courses(
    db: &DatabaseConnection,
    user_id: &UserId,
) -> usize {
    enrollments::Entity::find()
        .filter(enrollments::Column::StudentId.eq(user_id.to_string()))
        .filter(enrollments::Column::Status.eq("enrolled"))
        .count(db)
        .await
        .ok()
        .and_then(|v| v.try_into().ok())
        .unwrap_or(0)
}
