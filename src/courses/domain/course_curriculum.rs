use crate::courses::infrastructure::db::sea_orm_active_enums::CurriculumType;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CourseCurriculum {
    M2015,
    M2018,
    M2023,
}

//value

impl CourseCurriculum {
    pub fn value(&self) -> CurriculumType {
        match self {
            CourseCurriculum::M2015 => CurriculumType::M2015,
            CourseCurriculum::M2018 => CurriculumType::M2018,
            CourseCurriculum::M2023 => CurriculumType::M2023,
        }
    }

    //new

    pub fn new(value: CurriculumType) -> Result<Self, String> {
        match value {
            CurriculumType::M2015 => Ok(CourseCurriculum::M2015),
            CurriculumType::M2018 => Ok(CourseCurriculum::M2018),
            CurriculumType::M2023 => Ok(CourseCurriculum::M2023),
        }
    }

    pub fn is_valid(&self) -> bool {
        matches!(self, CourseCurriculum::M2015 | CourseCurriculum::M2018 | CourseCurriculum::M2023)
    }
}