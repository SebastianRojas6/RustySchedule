use sea_orm::*;
use crate::enrollment::domain::*;
use crate::enrollment::infrastructure::entity::enrollments;
use crate::enrollment::infrastructure::entity::courses;

pub async fn total_credits_enrolled(
    db: &DatabaseConnection,
    student_id: &UserId,
) -> CreditAmount {
    use crate::enrollment::infrastructure::entity::sea_orm_active_enums::EnrollmentStatus;

    let rows = enrollments::Entity::find()
        .filter(
            enrollments::Column::StudentId.eq(student_id.value())
                .and(enrollments::Column::Status.eq(EnrollmentStatus::Enrolled)),
        )
        .find_also_related(courses::Entity)
        .all(db)
        .await
        .unwrap_or_default();

    let total: i32 = rows
        .into_iter()
        .filter_map(|(_enrollment, maybe_course)| maybe_course.map(|c| c.credits))
        .sum();

    CreditAmount(total as u8)
}
