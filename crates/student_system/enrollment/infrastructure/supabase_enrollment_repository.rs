
/* 
use sea_orm::*;
use crate::enrollment::domain::*;
use crate::enrollment::infrastructure::entity::enrollments;
use shared::db_config::connect_to_supabase;
 
pub struct SupabaseEnrollmentRepository {
    pub db: DatabaseConnection,
}

impl SupabaseEnrollmentRepository {
    pub async fn new() -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        let db = connect_to_supabase().await?;
        Ok(Self { db })
    }
}

#[async_trait::async_trait]
impl EnrollmentRepository for SupabaseEnrollmentRepository {
    async fn find_by_id(&self, id: &EnrollmentId) -> Option<Enrollment> {
        let result = enrollments::Entity::find_by_id(id.value().clone())
            .one(&self.db)
            .await
            .ok()??;

        Some(Enrollment {
            id: EnrollmentId::new(result.id),
            student_id: UserId::new(result.student_id),
            course_id: CourseId::new(result.course_id),
            status: match result.status.as_str() {
                "Enrolled" => EnrollmentStatus::Enrolled,
                "Dropped" => EnrollmentStatus::Dropped,
                "Completed" => EnrollmentStatus::Completed,
                _ => return None,
            },
        })
    }

    async fn save(
        &self,
        enrollment: &Enrollment,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let model = enrollments::ActiveModel {
            id: Set(enrollment.id.value().clone()),
            student_id: Set(enrollment.student_id.value().clone()),
            course_id: Set(enrollment.course_id.value().clone()),
            status: Set(enrollment.status.to_string()), // Debe implementar Display
            ..Default::default()
        };

        enrollments::Entity::insert(model)
            .on_conflict(
                OnConflict::column(enrollments::Column::Id)
                    .update_columns([enrollments::Column::Status])
                    .to_owned(),
            )
            .exec(&self.db)
            .await?;

        Ok(())
    }

}

*/