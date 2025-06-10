use crate::enrollment::domain::*;
use super::super::use_cases::*;

use crate::enrollment::infrastructure::entity::enrollments as enrollment_entity;
use crate::enrollment::infrastructure::entity::courses as course_entity;

use sea_orm::{
    ColumnTrait, EntityTrait, QueryFilter, QuerySelect, JoinType, RelationTrait, Condition,
};

use sea_orm::PaginatorTrait;


use crate::enrollment::infrastructure::entity::sea_orm_active_enums::EnrollmentStatus as InfraStatus;

pub struct SupabaseEnrollmentRepository {
    pub db: sea_orm::DatabaseConnection,
}

impl SupabaseEnrollmentRepository {
    pub async fn new() -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        let db = shared::config::connect_to_supabase().await?;
        Ok(Self { db })
    }
}

#[async_trait::async_trait]
impl EnrollmentRepository for SupabaseEnrollmentRepository {
    async fn find_by_id(&self, id: &EnrollmentId) -> Option<Enrollment> {
        find_by_id::find_by_id(&self.db, id).await
    }

    async fn save(&self, enrollment: &Enrollment) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        save::save(&self.db, enrollment).await
    }

    async fn delete(&self, id: &EnrollmentId) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        use enrollment_entity::Entity as Enrollment;

        let result = Enrollment::delete_many()
            .filter(enrollment_entity::Column::Id.eq(id.to_string()))
            .exec(&self.db)
            .await?;

        if result.rows_affected == 0 {
            Err("Enrollment not found".into())
        } else {
            Ok(())
        }
    }

     async fn completed_courses(&self, student_id: &UserId) -> Vec<CourseId> {
        use enrollment_entity::Entity as Enrollment;

        let results = Enrollment::find()
            .filter(
                Condition::all()
                    .add(enrollment_entity::Column::StudentId.eq(student_id.to_string()))
                    .add(enrollment_entity::Column::Status.eq("Completed"))
            )
            .all(&self.db)
            .await;

        match results {
            Ok(enrollments) => enrollments
                .into_iter()
                .map(|e| CourseId::new(e.course_id))
                .collect(),
            Err(err) => {
                eprintln!("[ERROR] completed_courses: {}", err);
                vec![]
            }
        }
    }

    async fn current_enrollments(&self, student_id: &UserId) -> Vec<Enrollment> {
        use enrollment_entity::Entity as Enrollment;

        let results = Enrollment::find()
            .filter(enrollment_entity::Column::StudentId.eq(student_id.value()))
            .filter(enrollment_entity::Column::Status.eq(InfraStatus::Enrolled))
            .all(&self.db)
            .await;

        match results {
            Ok(models) => models
                .into_iter()
                .filter_map(|model| model.try_into().ok())
                .collect(),
            Err(err) => {
                eprintln!("[WARN] Error al obtener matrículas actuales: {:?}", err);
                vec![]
            }
        }
    }

    async fn find_by_student_and_course(&self, student_id: &UserId, course_id: &CourseId) -> Option<Enrollment> {
        use enrollment_entity::Entity as Enrollment;

        let result = Enrollment::find()
            .filter(enrollment_entity::Column::StudentId.eq(student_id.value()))
            .filter(enrollment_entity::Column::CourseId.eq(course_id.value()))
            .one(&self.db)
            .await;

        match result {
            Ok(Some(model)) => model.try_into().ok(),
            Ok(None) => None,
            Err(err) => {
                eprintln!("[WARN] Error al buscar matrícula por estudiante y curso: {:?}", err);
                None
            }
        }
    }

    async fn total_credits_enrolled(&self, student_id: &UserId) -> CreditAmount {
        use enrollment_entity::Entity as Enrollment;

        let enrollments = Enrollment::find()
            .filter(enrollment_entity::Column::StudentId.eq(student_id.value()))
            .filter(enrollment_entity::Column::Status.eq(InfraStatus::Enrolled))
            .join(JoinType::InnerJoin, enrollment_entity::Relation::Courses.def())
            .select_only()
            .column(course_entity::Column::Credits)
            .into_tuple::<i32>()
            .all(&self.db)
            .await;

        match enrollments {
            Ok(credits) => {
                let total: i32 = credits.into_iter().sum();
                CreditAmount::new(total as u8)
            }
            Err(err) => {
                eprintln!("[WARN] Error al calcular créditos totales: {:?}", err);
                CreditAmount::new(0)
            }
        }
    }

    async fn enrollment_attempts(&self, student_id: &UserId, course_id: &CourseId) -> u8 {
        use enrollment_entity::Entity as Enrollment;

        let result = Enrollment::find()
            .filter(
                Condition::all()
                    .add(enrollment_entity::Column::StudentId.eq(student_id.to_string()))
                    .add(enrollment_entity::Column::CourseId.eq(course_id.to_string()))
            )
            .count(&self.db)
            .await;

        match result {
            Ok(count) => count as u8,
            Err(err) => {
                eprintln!(
                    "[WARN] Error al contar intentos de matrícula: {} (student_id: {:?}, course_id: {:?})",
                    err,
                    student_id,
                    course_id
                );
                0
            }
        }
    }
}
