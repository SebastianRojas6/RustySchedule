/* 

use sea_orm::{ColumnTrait, EntityTrait, JoinType, QueryFilter, QuerySelect, RelationTrait};
// use crate;
use crate::enrollment::infrastructure::entity::{enrollments, courses, course_schedules};
use sea_orm::DatabaseConnection;

pub async fn get_schedule(
    db: &DatabaseConnection,
    student_id: &str,
) -> StudentSchedule {
    let tuples = enrollments::Entity::find()
        .filter(enrollments::Column::StudentId.eq(student_id.to_string()))
        .join(JoinType::InnerJoin, enrollments::Relation::Courses.def())
        .join(JoinType::InnerJoin, courses::Relation::CourseSchedules.def())
        .select_only()
        .column(courses::Column::Id)
        .column(courses::Column::Name)
        .column(course_schedules::Column::Day)
        .column(course_schedules::Column::StartTime)
        .column(course_schedules::Column::EndTime)
        .into_tuple::<(String, String, String, String, String)>()
        .all(db)
        .await
        .unwrap_or_default();

    let entries = tuples
        .into_iter()
        .map(|(course_id, course_name, day, start_time, end_time)| ScheduleEntry {
            course_id,
            course_name,
            day,
            start_time,
            end_time,
        })
        .collect();

    StudentSchedule {
        student_id: student_id.to_string(),
        entries,
    }
}


*/