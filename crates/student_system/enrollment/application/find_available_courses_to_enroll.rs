use crate::enrollment::domain::{
    repository::EnrollmentRepository,
    user_id::UserId,
    available_course::AvailableCourse,
    course_id::CourseId,
};

pub struct FindAvailableCoursesToEnrollUseCase<'a> {
    repo: &'a dyn EnrollmentRepository,
}

impl<'a> FindAvailableCoursesToEnrollUseCase<'a> {
    pub fn new(repo: &'a dyn EnrollmentRepository) -> Self {
        Self { repo }
    }

    pub async fn execute(&self, student_id: &UserId) -> Result<Vec<AvailableCourse>, String> {
        let completed = self.repo.completed_courses(student_id).await?;
        let enrolled = self.repo.current_enrollments(student_id).await?;

        if self.repo.find_user_info_by_id(student_id).await.is_none() {
            return Err("Usuario no encontrado".into());
        }

        let Some(current_semester) = self.repo.find_any_enrolled_semester(student_id).await else {
            return Err("No hay semestre activo".into());
        };

        let all_courses = self.repo
            .find_active_courses_by_semester(&current_semester)
            .await?;

        let mut available = Vec::new();

        'course: for course in all_courses {
            let course_id = CourseId(course.id.clone());

            let already_taken = completed.contains(&course_id)
                || enrolled.iter().any(|e| e.course_id == course_id);

            if already_taken {
                continue;
            }

            let prerequisites = self
                .repo
                .find_prerequisites_by_course_id(&course.id)
                .await?;

            for prereq in prerequisites {
                if !completed.contains(&prereq) {
                    continue 'course;
                }
            }

            let num_sections = self
                .repo
                .count_sections_by_course_code(&course.code)
                .await?;

            available.push(AvailableCourse {
                code: course.code.clone(),
                id: course.id.clone(),
                name: course.name,
                cycle: course.cycle,
                credits: course.credits,
                num_sections,
            });
        }

        Ok(available)
    }
}
