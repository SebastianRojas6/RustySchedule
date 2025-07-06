use crate::domain::{
    models::enums::{SessionType, Weekday},
    models::schedule::Schedule,
    repositories::{course_repository::CourseRepository, schedule_repository::ScheduleRepository},
};
use chrono::NaiveTime;
use std::collections::VecDeque;
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

        // 3. Calcular horas pendientes por asignar por curso
        let mut courses_with_pending_hours: VecDeque<_> = courses
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

        // 4. Generar posibles horarios
        let mut suggested_schedules = Vec::new();
        let days = [Weekday::Monday, Weekday::Tuesday, Weekday::Wednesday, Weekday::Thursday, Weekday::Friday, Weekday::Saturday];

        let min_time = NaiveTime::from_hms_opt(8, 0, 0).unwrap();
        let max_time = NaiveTime::from_hms_opt(22, 0, 0).unwrap();
        let time_slot_duration = chrono::Duration::hours(1);

        while let Some((course, pending_hours)) = courses_with_pending_hours.pop_front() {
            let mut hours_assigned = 0;

            // Intentar asignar bloques de 2 horas preferentemente
            let preferred_duration = chrono::Duration::hours(2).min(chrono::Duration::hours(pending_hours as i64));

            'day_loop: for &day in &days {
                let mut current_time = min_time;

                while current_time + preferred_duration <= max_time {
                    let end_time = current_time + preferred_duration;
                    let test_schedule = Schedule {
                        id: "temp".to_string(),
                        course_id: course.id.clone(),
                        day,
                        start_time: current_time,
                        end_time,
                        session_type: SessionType::Theory, // Tipo por defecto
                        location_detail: None,
                        created_at: None,
                        facility_id: "".to_string(), // Se asignará después
                    };

                    // Verificar conflicto con horarios existentes
                    let has_conflict = existing_schedules.iter().chain(suggested_schedules.iter()).any(|s| s.conflicts_with(&test_schedule));

                    if !has_conflict {
                        // Validar que el profesor no tenga otro curso a esta hora
                        let is_valid = self.validate_schedule(teacher_id, &test_schedule).await?;

                        if is_valid {
                            let mut new_schedule = test_schedule.clone();
                            new_schedule.id = format!("suggested-{}", suggested_schedules.len() + 1);
                            suggested_schedules.push(new_schedule);
                            hours_assigned += preferred_duration.num_hours() as i32;

                            if hours_assigned >= pending_hours {
                                break 'day_loop;
                            }
                        }
                    }

                    current_time += time_slot_duration;
                }
            }

            // Si no asignó todas las horas, volver a poner en la cola
            if hours_assigned < pending_hours {
                courses_with_pending_hours.push_back((course, pending_hours - hours_assigned));
            }
        }

        // 5. Si aún quedan horas por asignar, intentar con bloques más pequeños
        if !courses_with_pending_hours.is_empty() {
            for (course, pending_hours) in courses_with_pending_hours {
                let mut hours_assigned = 0;

                'day_loop: for &day in &days {
                    let mut current_time = min_time;

                    while current_time + time_slot_duration <= max_time {
                        let end_time = current_time + time_slot_duration;
                        let test_schedule = Schedule {
                            id: "temp".to_string(),
                            course_id: course.id.clone(),
                            day,
                            start_time: current_time,
                            end_time,
                            session_type: SessionType::Theory,
                            location_detail: None,
                            created_at: None,
                            facility_id: "".to_string(),
                        };

                        let has_conflict = existing_schedules.iter().chain(suggested_schedules.iter()).any(|s| s.conflicts_with(&test_schedule));

                        if !has_conflict && self.validate_schedule(teacher_id, &test_schedule).await? {
                            let mut new_schedule = test_schedule.clone();
                            new_schedule.id = format!("suggested-{}", suggested_schedules.len() + 1);
                            suggested_schedules.push(new_schedule);
                            hours_assigned += 1;

                            if hours_assigned >= pending_hours {
                                break 'day_loop;
                            }
                        }

                        current_time += time_slot_duration;
                    }
                }
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
