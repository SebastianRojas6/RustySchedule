use serde::Deserialize;

#[derive(Deserialize)]
pub struct EnrollInCourseRequestDto {
    pub student_id: String,
    pub course_id: String,
    pub student_curriculum: String,
    pub course_curriculum: String,
    pub student_status: String,
    pub student_credits_enrolled: u32,
    pub course_credits: u32,
    pub completed_courses: Vec<String>,
    pub course_prerequisites: Vec<String>,
    pub course_cycle: u8,
    pub semester: String,
    pub section_capacity_available: bool,
    pub already_enrolled: bool,
    pub times_repeated: u8,
    pub schedule_conflict: bool,
    pub course_already_passed: bool,
}
