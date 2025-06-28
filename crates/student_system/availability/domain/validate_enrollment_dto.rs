use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ValidateEnrollmentDto {
    pub student_id: String,
    pub course_id: String,
    pub section_id: String,
}
