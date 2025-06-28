use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct AvailableCourseDto {
    pub id: String,
    pub code: String,
    pub name: String,
    pub credits: i32,
    pub cycle: i32,
    pub teacher_id: String,
    pub section: i32,
    pub curriculum: String,
}

