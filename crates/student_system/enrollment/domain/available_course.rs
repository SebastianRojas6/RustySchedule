use serde::Serialize;

use crate::enrollment::infrastructure::entity::sea_orm_active_enums::{DayType, SessionType};

#[derive(Debug, Clone, Serialize)]
pub struct AvailableCourse {
    pub code: String,
    pub name: String,
    pub cycle: i32,
    pub credits: i32,
    pub num_sections: usize,
}

#[derive(Debug, Clone, Serialize)]
pub struct AvailableSection {
    pub available_spots: i32,
    pub teacher_id: String,
    pub day: DayType,
    pub session_type: SessionType,
    pub start_time: String, 
    pub end_time: String,
}

pub struct CourseWithSections {
    pub nombre_curso: String,
    pub codigo: String,
    pub num_ciclo: i32,
    pub creditos: i32,
    pub secciones: Vec<AvailableSection>,
}

#[derive(Debug, Clone, Serialize)]
pub struct CourseSections {
    pub nombre_curso: String,
    pub codigo: String,
    pub num_ciclo: i32,
    pub creditos: i32,
    pub num_secc: usize,
    pub secciones: Vec<AvailableSection>,
}

#[derive(Debug, Clone)]
pub struct CourseData {
    pub id: String,
    pub code: String,
    pub name: String,
    pub cycle: i32,
    pub credits: i32,
}


#[derive(Debug, Clone)]
pub struct SectionData {
    pub code: String,
    pub name: String,
    pub cycle: i32,
    pub credits: i32,
    pub available_spots: i32,
    pub teacher_id: String,
    pub day: DayType,
    pub session_type: SessionType,
    pub start_time: String,
    pub end_time: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CourseCode(pub String);
