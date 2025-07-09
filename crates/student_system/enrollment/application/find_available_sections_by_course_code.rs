use crate::enrollment::domain::{
    repository::EnrollmentRepository,
    available_course::{AvailableSection, CourseSections},
};

pub struct FindAvailableSectionsByCourseCodeUseCase<'a> {
    repo: &'a dyn EnrollmentRepository,
}

impl<'a> FindAvailableSectionsByCourseCodeUseCase<'a> {
    pub fn new(repo: &'a dyn EnrollmentRepository) -> Self {
        Self { repo }
    }

    pub async fn execute(&self, course_id: &str) -> Result<CourseSections, String> {
        let course_with_sections = self.repo.find_sections_by_course_code(course_id).await?;

        let available_sections: Vec<AvailableSection> = course_with_sections
            .secciones
            .into_iter()
            .filter(|s| s.available_spots > 0)
            .collect();

        Ok(CourseSections {
            id: course_with_sections.id.clone(),
            nombre_curso: course_with_sections.nombre_curso,
            codigo: course_with_sections.codigo,
            num_ciclo: course_with_sections.num_ciclo,
            creditos: course_with_sections.creditos,
            num_secc: available_sections.len(),
            secciones: available_sections,
        })
    }
}