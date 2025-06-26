use sea_orm::{EntityTrait, ColumnTrait, QueryFilter, JoinType, QuerySelect, DbConn,RelationTrait};
use crate::enrollment::infrastructure::entity::{enrollments, courses};
use crate::crud_enrollment::domain::enrollment_dto::EnrolledCourseDto;

pub async fn get_schedule(conn: &DbConn, student_id: &str) -> Result<Vec<EnrolledCourseDto>, sea_orm::DbErr> {
    let results = enrollments::Entity::find()
        .filter(enrollments::Column::StudentId.eq(student_id))
        .join(JoinType::InnerJoin, enrollments::Relation::Courses.def())
        .select_also(courses::Entity)
        .all(conn)
        .await?;

    let dto_list = results.into_iter().filter_map(|(enrollment, course)| {
        course.map(|c| EnrolledCourseDto {
            course_id: enrollment.course_id,
            name: c.name,
            status: enrollment.status.to_string(), 
            credits: c.credits,
        })
    }).collect();

    Ok(dto_list)
}
