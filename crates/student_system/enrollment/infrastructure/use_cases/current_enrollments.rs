use sea_orm::*;
use crate::enrollment::domain::*;
use crate::enrollment::infrastructure::entity::enrollments;
use crate::enrollment::infrastructure::entity::sea_orm_active_enums::EnrollmentStatus as InfraStatus;

pub async fn current_enrollments(
    db: &DatabaseConnection,
    student_id: &UserId,
) -> Result<Vec<Enrollment>, String> {
    let rows = enrollments::Entity::find()
        .filter(
            enrollments::Column::StudentId.eq(student_id.value())
                .and(enrollments::Column::Status.eq(InfraStatus::Enrolled)),
        )
        .all(db)
        .await;

    match rows {
        Ok(models) => {
            let enrollments = models
                .into_iter()
                .filter_map(|model| model.try_into().ok()) // puedes mejorar esto después
                .collect();
            Ok(enrollments)
        }
        Err(err) => {
            eprintln!("[ERROR] current_enrollments: {}", err);
            Err("Failed to fetch enrollments".into())
        }
    }
}

