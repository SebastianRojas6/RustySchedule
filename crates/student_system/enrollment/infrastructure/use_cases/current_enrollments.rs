use sea_orm::*;
use crate::enrollment::domain::*;
use crate::enrollment::infrastructure::entity::enrollments;
use crate::enrollment::infrastructure::entity::sea_orm_active_enums::EnrollmentStatus as InfraStatus;

pub async fn current_enrollments(
    db: &DatabaseConnection,
    student_id: &UserId,
) -> Vec<Enrollment> {
    let rows = enrollments::Entity::find()
        .filter(
            enrollments::Column::StudentId.eq(student_id.value())
                .and(enrollments::Column::Status.eq(InfraStatus::Enrolled)),
        )
        .all(db)
        .await;

    match rows {
        Ok(models) => models
            .into_iter()
            .filter_map(|model| model.try_into().ok())
            .collect(),
        Err(err) => {
            eprintln!("[ERROR] current_enrollments: {}", err);
            vec![]
        }
    }
}
