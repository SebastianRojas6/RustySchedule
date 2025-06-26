use serde::Serialize;

#[derive(Serialize)]
pub struct EnrolledCourseDto {
    pub course_id: String,
    pub name: String,
    pub status: String,
    pub credits: i32,
}
