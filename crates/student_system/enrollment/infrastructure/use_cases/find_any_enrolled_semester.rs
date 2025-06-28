use sea_orm::*;
use serde_json::Value;
use crate::enrollment::domain::UserId;
use crate::enrollment::infrastructure::entity::{enrollments, courses};

pub async fn find_any_enrolled_semester(
    db: &DatabaseConnection,
    user_id: &UserId,
) -> Option<String> {
    let json: Value = enrollments::Entity::find()
        .filter(enrollments::Column::StudentId.eq(user_id.to_string()))
        .filter(enrollments::Column::Status.eq("enrolled"))
        .join(JoinType::InnerJoin, enrollments::Relation::Courses.def())
        .select_only()
        .column(courses::Column::Semester)
        .into_json()
        .one(db)
        .await
        .ok()??;

    json.get("semester")?.as_str().map(|s| s.to_string())
}
