use sea_orm::*;
use crate::enrollment::domain::*;
use crate::enrollment::infrastructure::entity::enrollments;
use crate::enrollment::infrastructure::entity::sea_orm_active_enums::EnrollmentStatus as DbEnrollmentStatus;

pub async fn save(
    db: &DatabaseConnection,
    enrollment: &Enrollment,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let model = enrollments::ActiveModel {
        id: Set(enrollment.id.value().to_string()),
        student_id: Set(enrollment.student_id.value().to_string()),
        course_id: Set(enrollment.course_id.value().to_string()),
        status: Set(DbEnrollmentStatus::from(enrollment.status.clone())),
        ..Default::default()
    };

    enrollments::Entity::insert(model)
        .on_conflict(
            sea_orm::sea_query::OnConflict::column(enrollments::Column::Id)
                .update_columns([enrollments::Column::Status])
                .to_owned(),
        )
        .exec(db)
        .await?;

    Ok(())
}
