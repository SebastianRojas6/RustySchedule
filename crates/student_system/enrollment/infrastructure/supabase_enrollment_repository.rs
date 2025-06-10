use sea_orm::*;
use sea_orm::sea_query::OnConflict;
use crate::enrollment::domain::*;
use crate::enrollment::infrastructure::entity::enrollments;
use shared::config::connect_to_supabase;

use crate::enrollment::domain::EnrollmentStatus as DomainEnrollmentStatus;
use crate::enrollment::infrastructure::entity::sea_orm_active_enums::EnrollmentStatus as DbEnrollmentStatus;

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
        let result = enrollments::Entity::find_by_id(id.value())
            .one(&self.db)
            .await
            .ok()??;

        Some(Enrollment {
            id: EnrollmentId::new(result.id),
            student_id: UserId::new(result.student_id),
            course_id: CourseId::new(result.course_id),
            status: DomainEnrollmentStatus::try_from(result.status).ok()?,
        })
    }

    async fn save(
        &self,
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
                OnConflict::column(enrollments::Column::Id)
                    .update_columns([enrollments::Column::Status])
                    .to_owned(),
            )
            .exec(&self.db)
            .await?;

        Ok(())
    }
    
    async fn find_by_student_and_course(&self, student_id: &UserId, course_id: &CourseId) -> Option<Enrollment> {
        let result = enrollments::Entity::find()
            .filter(
                enrollments::Column::StudentId.eq(student_id.value())
                    .and(enrollments::Column::CourseId.eq(course_id.value())),
            )
            .one(&self.db)
            .await
            .ok()??;

        Some(Enrollment {
            id: EnrollmentId::new(result.id),
            student_id: UserId::new(result.student_id),
            course_id: CourseId::new(result.course_id),
            status: DomainEnrollmentStatus::try_from(result.status).ok()?,
        })
    }

    async fn delete(&self, id: &EnrollmentId) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        enrollments::Entity::delete_by_id(id.value())
            .exec(&self.db)
            .await?;
        Ok(())
    }

    async fn total_credits_enrolled(&self, _student_id: &UserId) -> CreditAmount {
        CreditAmount(0) //por ahora
    }

    async fn completed_courses(&self, student_id: &UserId) -> Vec<CourseId> {
        let rows = enrollments::Entity::find()
            .filter(
                enrollments::Column::StudentId.eq(student_id.value())
                    .and(enrollments::Column::Status.eq(DbEnrollmentStatus::Completed)),
            )
            .all(&self.db)
            .await
            .unwrap_or_default();

        rows.into_iter()
            .map(|r| CourseId::new(r.course_id))
            .collect()
    }

    async fn enrollment_attempts(&self, student_id: &UserId, course_id: &CourseId) -> u8 {
        let count = enrollments::Entity::find()
            .filter(
                enrollments::Column::StudentId.eq(student_id.value())
                    .and(enrollments::Column::CourseId.eq(course_id.value())),
            )
            .count(&self.db)
            .await
            .unwrap_or(0);

        count as u8
    }

    async fn current_enrollments(&self, student_id: &UserId) -> Vec<Enrollment> {
        let rows = enrollments::Entity::find()
            .filter(
                enrollments::Column::StudentId.eq(student_id.value())
                    .and(enrollments::Column::Status.eq(DbEnrollmentStatus::Enrolled)),
            )
            .all(&self.db)
            .await
            .unwrap_or_default();

        rows.into_iter()
            .filter_map(|r| {
                Some(Enrollment {
                    id: EnrollmentId::new(r.id),
                    student_id: UserId::new(r.student_id),
                    course_id: CourseId::new(r.course_id),
                    status: DomainEnrollmentStatus::try_from(r.status).ok()?,
                })
            })
            .collect()
    }
}
