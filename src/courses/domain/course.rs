pub use super::{
    course_capacity::CourseCapacity, course_code::CourseCode, course_credits::CourseCredits, course_curriculum::CourseCurriculum, course_cycle::CourseCycle, course_name::CourseName,
    course_schedule::Schedule, course_section::CourseSection,
};

#[derive(Debug, Clone)]
pub struct Course {
    pub code: CourseCode,
    pub name: CourseName,
    pub section: CourseSection,
    pub curriculum: CourseCurriculum,
    pub capacity: CourseCapacity,
    pub credits: CourseCredits,
    pub prerequisites: Vec<String>, // IDs de cursos como strings
    pub teacher_id: String,
    pub facility_id: String,
    pub cycle: CourseCycle,
    pub enrolled: i32,
    pub schedule: Vec<Schedule>,
}
