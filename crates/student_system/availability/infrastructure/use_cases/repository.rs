use sea_orm::{
    RelationTrait, PaginatorTrait, EntityTrait, ColumnTrait, QueryFilter, QuerySelect, EnumIter, DeriveColumn, JoinType
};

use crate::enrollment::infrastructure::entity::{users, courses, enrollments, course_prerequisites};
use crate::enrollment::domain::*;
use crate::availability::domain::repository::EnrollmentAvailabilityRepository;

pub struct SupabaseAvailabilityRepository {
    pub db: sea_orm::DatabaseConnection,
}

impl SupabaseAvailabilityRepository {
    pub async fn new() -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        let db = shared::config::connect_to_supabase().await?;
        Ok(Self { db })
    }
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveColumn)]
enum QueryAs {
    TotalCredits,
}

#[async_trait::async_trait]
impl EnrollmentAvailabilityRepository for SupabaseAvailabilityRepository {
    async fn get_student_curriculum(&self, _student_id: &UserId) -> anyhow::Result<CurriculumId> {
        let user = users::Entity::find_by_id(_student_id.value().to_string())
            .one(&self.db).await?
            .ok_or_else(|| anyhow::anyhow!("Student not found"))?;

        Ok(CurriculumId::new(user.specialty))
    }

    async fn get_course_curriculum(&self, _course_id: &CourseId) -> anyhow::Result<CurriculumId> {
        let course = courses::Entity::find_by_id(_course_id.value().to_string())
            .one(&self.db).await?
            .ok_or_else(|| anyhow::anyhow!("Course not found"))?;

        Ok(CurriculumId::new(course.curriculum.to_string()))
    }

    async fn get_student_status(&self, _student_id: &UserId) -> anyhow::Result<StudentStatus> {
        let user = users::Entity::find_by_id(_student_id.value().to_string())
            .one(&self.db).await?
            .ok_or_else(|| anyhow::anyhow!("Student not found"))?;
        let status: StudentStatus = user.student_status
            .ok_or_else(|| anyhow::anyhow!("Student status missing"))?
            .into();
        Ok(status)
    }

    async fn get_student_enrolled_credits(&self, student_id: &UserId) -> anyhow::Result<CreditAmount> {
        let result: Option<i64> = enrollments::Entity::find()
            .filter(enrollments::Column::StudentId.eq(student_id.value().to_string()))
            .filter(enrollments::Column::Status.eq("enrolled"))
            .join(JoinType::InnerJoin, enrollments::Relation::Courses.def())
            .select_only()
            .column_as(courses::Column::Credits.sum(), QueryAs::TotalCredits)
            .into_values::<_, QueryAs>()
            .one(&self.db)
            .await?;

    let total_credits = result.unwrap_or(0);

    Ok(CreditAmount::new(total_credits as u8))
}

    async fn get_course_credits(&self, _course_id: &CourseId) -> anyhow::Result<CreditAmount> {
        let course = courses::Entity::find_by_id(_course_id.value().to_string())
            .one(&self.db).await?
            .ok_or_else(|| anyhow::anyhow!("Course not found"))?;

        Ok(CreditAmount::new(course.credits as u8))
    }

    async fn get_completed_courses(&self, _student_id: &UserId) -> anyhow::Result<Vec<CourseId>> {
        let completed = enrollments::Entity::find()
            .filter(enrollments::Column::StudentId.eq(_student_id.value().to_string()))
            .filter(enrollments::Column::Status.eq("completed"))
            .all(&self.db)
            .await?;

        Ok(completed.into_iter().map(|e| CourseId::new(e.course_id)).collect())
    }

    async fn get_course_prerequisites(&self, _course_id: &CourseId) -> anyhow::Result<Vec<CourseId>> {
        let prereqs = course_prerequisites::Entity::find()
            .filter(course_prerequisites::Column::CourseId.eq(_course_id.value().to_string()))
            .all(&self.db).await?;

        Ok(prereqs.into_iter().map(|p| CourseId::new(p.prerequisite_course_id)).collect())
    }

    async fn get_course_cycle(&self, _course_id: &CourseId) -> anyhow::Result<CourseCycle> {
        let course = courses::Entity::find_by_id(_course_id.value().to_string())
            .one(&self.db).await?
            .ok_or_else(|| anyhow::anyhow!("Course not found"))?;

        Ok(CourseCycle::new(course.cycle as u8))
    }

    async fn get_course_semester_parity(&self, _course_id: &CourseId) -> anyhow::Result<SemesterParity> {
        let course = courses::Entity::find_by_id(_course_id.value().to_string())
            .one(&self.db).await?
            .ok_or_else(|| anyhow::anyhow!("Course not found"))?;

        if course.cycle % 2 == 0 {
            Ok(SemesterParity::Even)
        } else {
            Ok(SemesterParity::Odd)
        }
    }

    async fn is_section_capacity_available(&self, _course_id: &CourseId, _section_id: &str) -> anyhow::Result<bool> {
        let course = courses::Entity::find()
            .filter(courses::Column::Id.eq(_course_id.value().to_string()))
            .filter(courses::Column::Section.eq(_section_id.parse::<i32>()?))
            .one(&self.db).await?
            .ok_or_else(|| anyhow::anyhow!("Course not found"))?;

        Ok(course.available_spots.unwrap_or(0) > 0)
    }

    async fn is_student_already_enrolled(&self, _student_id: &UserId, _course_id: &CourseId) -> anyhow::Result<bool> {
        let exists = enrollments::Entity::find()
            .filter(enrollments::Column::StudentId.eq(_student_id.value().to_string()))
            .filter(enrollments::Column::CourseId.eq(_course_id.value().to_string()))
            .filter(enrollments::Column::Status.eq("enrolled"))
            .count(&self.db)
            .await?;

        Ok(exists > 0)
    }

    async fn count_course_repetitions(&self, _student_id: &UserId, _course_id: &CourseId) -> anyhow::Result<u8> {
        let count = enrollments::Entity::find()
            .filter(enrollments::Column::StudentId.eq(_student_id.value().to_string()))
            .filter(enrollments::Column::CourseId.eq(_course_id.value().to_string()))
            .count(&self.db)
            .await?;

        Ok(count as u8)
    }

    async fn has_schedule_conflict(&self, _student_id: &UserId, _course_id: &CourseId, _section_id: &str) -> anyhow::Result<bool> {
        use crate::enrollment::infrastructure::entity::course_schedules;

        let target_schedules = course_schedules::Entity::find()
            .filter(course_schedules::Column::CourseId.eq(_course_id.value().to_string()))
            .all(&self.db)
            .await?;

        let current_courses = enrollments::Entity::find()
            .filter(enrollments::Column::StudentId.eq(_student_id.value().to_string()))
            .filter(enrollments::Column::Status.eq("enrolled"))
            .all(&self.db)
            .await?;

        let current_course_ids: Vec<String> = current_courses.iter().map(|e| e.course_id.clone()).collect();

        let student_schedules = course_schedules::Entity::find()
            .filter(course_schedules::Column::CourseId.is_in(current_course_ids))
            .all(&self.db)
            .await?;

        for t in &target_schedules {
            for s in &student_schedules {
                if t.day == s.day
                    && !(t.end_time <= s.start_time || t.start_time >= s.end_time)
                {
                    return Ok(true);
                }
            }
        }

        Ok(false)
    }

    async fn has_already_passed_course(&self, _student_id: &UserId, _course_id: &CourseId) -> anyhow::Result<bool> {
        let passed = enrollments::Entity::find()
            .filter(enrollments::Column::StudentId.eq(_student_id.value().to_string()))
            .filter(enrollments::Column::CourseId.eq(_course_id.value().to_string()))
            .filter(enrollments::Column::Status.eq("completed"))
            .count(&self.db)
            .await?;

        Ok(passed > 0)
    }
}
