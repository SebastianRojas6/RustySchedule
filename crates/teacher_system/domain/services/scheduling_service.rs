use crate::domain::{
    models::enums::{SessionType, Weekday},
    models::schedule::Schedule,
    repositories::{course_repository::CourseRepository, schedule_repository::ScheduleRepository},
};
use chrono::NaiveTime;
use std::sync::Arc;

#[derive(Clone)]
pub struct DefaultSchedulingService {
    course_repo: Arc<dyn CourseRepository + Send + Sync>,
    schedule_repo: Arc<dyn ScheduleRepository + Send + Sync>,
}

impl DefaultSchedulingService {
    pub fn new(course_repo: Arc<dyn CourseRepository + Send + Sync>, schedule_repo: Arc<dyn ScheduleRepository + Send + Sync>) -> Self {
        Self { course_repo, schedule_repo }
    }
}

impl DefaultSchedulingService {
    pub async fn suggest_available_time(&self, teacher_id: &str) -> Result<Vec<Schedule>, String> {
        // 1. Obtener todos los cursos del profesor
        let courses = self.course_repo.get_courses_by_user(teacher_id).await?;

        // 2. Obtener los horarios existentes del profesor
        let mut existing_schedules = Vec::new();
        for course in &courses {
            let schedules = self.schedule_repo.get_schedules_by_course(&course.id).await?;
            existing_schedules.extend(schedules);
        }

        // 3. Ordenar cursos por horas pendientes (de mayor a menor)
        let mut courses_with_pending_hours: Vec<_> = courses
            .iter()
            .map(|course| {
                let assigned_hours: i32 = existing_schedules
                    .iter()
                    .filter(|s| s.course_id == course.id)
                    .map(|s| (s.end_time - s.start_time).num_hours() as i32)
                    .sum();

                (course, course.hours_per_week - assigned_hours)
            })
            .filter(|(_, pending)| *pending > 0)
            .collect();

        // Ordenar por horas pendientes (mayor primero)
        courses_with_pending_hours.sort_by(|a, b| b.1.cmp(&a.1));

        // 4. Generar posibles horarios
        let mut suggested_schedules = Vec::new();
        let days = [Weekday::Monday, Weekday::Tuesday, Weekday::Wednesday, Weekday::Thursday, Weekday::Friday, Weekday::Saturday];

        let min_time = NaiveTime::from_hms_opt(8, 0, 0).unwrap();
        let max_time = NaiveTime::from_hms_opt(22, 0, 0).unwrap();

        for (course, pending_hours) in courses_with_pending_hours {
            let duration = chrono::Duration::hours(pending_hours as i64);
            let mut schedule_found = false;

            'day_loop: for &day in &days {
                // Verificar si ya hay horarios sugeridos para este día
                let day_schedules: Vec<&Schedule> = suggested_schedules.iter().chain(existing_schedules.iter()).filter(|s| s.day == day).collect();

                // Empezar desde la hora mínima
                let mut current_time = min_time;

                while current_time + duration <= max_time {
                    let end_time = current_time + duration;
                    let test_schedule = Schedule {
                        id: "temp".to_string(),
                        course_id: course.id.clone(),
                        day,
                        start_time: current_time,
                        end_time,
                        session_type: SessionType::Theory, // Tipo por defecto
                        location_detail: None,
                        created_at: None,
                        facility_id: "".to_string(),
                    };

                    // Verificar si hay solapamiento con otros horarios
                    let has_conflict = day_schedules.iter().any(|s| s.conflicts_with(&test_schedule));

                    if !has_conflict {
                        // Validar que el profesor no tenga otro curso a esta hora
                        let is_valid = self.validate_schedule(teacher_id, &test_schedule).await?;

                        if is_valid {
                            let mut new_schedule = test_schedule.clone();
                            new_schedule.id = format!("suggested-{}", suggested_schedules.len() + 1);
                            suggested_schedules.push(new_schedule);
                            schedule_found = true;
                            break 'day_loop;
                        }
                    }

                    // Avanzar en bloques de 30 minutos para mayor precisión
                    current_time += chrono::Duration::minutes(30);
                }
            }

            if !schedule_found {
                return Err(format!(
                    "No se pudo encontrar un horario disponible para el curso {} que requiere {} horas continuas",
                    course.name, pending_hours
                ));
            }
        }

        Ok(suggested_schedules)
    }

    pub async fn validate_schedule(&self, teacher_id: &str, schedule: &Schedule) -> Result<bool, String> {
        let teacher_courses = self.course_repo.get_courses_by_user(&teacher_id).await?;

        for course in teacher_courses {
            let existing_schedule = self.schedule_repo.get_schedules_by_course(&course.id).await?.iter().any(|s| s.conflicts_with(schedule));

            if existing_schedule {
                return Ok(false);
            }
        }

        Ok(true)
    }
}
