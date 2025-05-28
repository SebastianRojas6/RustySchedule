use crate::courses::domain::course::Course;
use crate::courses::domain::course_code::CourseCode;
use crate::courses::domain::course_name::CourseName;
use crate::courses::domain::course_section::CourseSection;
use crate::courses::domain::course_curriculum::CourseCurriculum;
use crate::courses::domain::course_capacity::CourseCapacity;
use crate::courses::domain::course_credits::CourseCredits;
use crate::courses::domain::course_cycle::CourseCycle;

use crate::courses::infrastructure::db::courses::Model as CourseModel;

use crate::courses::application::mappers::db_mapper::DbMapper;

use sea_orm::JsonValue;

pub struct CourseDbMapper;

impl DbMapper<Course, CourseModel> for CourseDbMapper {
    fn to_db(entity: Course) -> CourseModel {
        CourseModel {
            id: format!("{}-{}", entity.code.value(), entity.section.value()), 
            code: entity.code.value().to_string(),
            name: entity.name.value().to_string(),
            section: entity.section.value(),
            curriculum: entity.curriculum.value().clone(),
            capacity: entity.capacity.value(),
            credits: entity.credits.value(),
            teacher_id: entity.teacher_id.clone(),
            facility_id: entity.facility_id.clone(),
            cycle: entity.cycle.value(),
            enrolled: entity.enrolled,
            prerequisites: serde_json::to_value(&entity.prerequisites).unwrap_or(JsonValue::Null),
        }
    }

    fn to_entity(model: CourseModel) -> Course {
        Course {
            code: CourseCode::new(model.code).expect("Invalid CourseCode"),
            name: CourseName::new(model.name).expect("Invalid CourseName"),
            section: CourseSection::new(model.section).expect("Invalid Section"),
            curriculum: CourseCurriculum::new(model.curriculum).expect("Invalid Curriculum"),
            capacity: CourseCapacity::new(model.capacity).expect("Invalid Capacity"),
            credits: CourseCredits::new(model.credits).expect("Invalid Credits"),
            teacher_id: model.teacher_id,
            facility_id: model.facility_id,
            cycle: CourseCycle::new(model.cycle).expect("Invalid Cycle"),
            enrolled: model.enrolled as i32,
            schedule: vec![],
            prerequisites: serde_json::from_value(model.prerequisites).unwrap_or_default(),
        }
    }
}
