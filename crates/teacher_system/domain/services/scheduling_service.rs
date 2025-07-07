use crate::{
    domain::{
        models::{
            enums::{SessionType, Weekday},
            facilitie_available::FacilityAvailable,
            schedule::Schedule,
        },
        repositories::{course_repository::CourseRepository, facility_repository::FacilityRepository, schedule_repository::ScheduleRepository},
    },
    infrastructure::database::queries::facility_query::SupabaseFacilityRepository,
};
use chrono::NaiveTime;
use std::{collections::BTreeMap, sync::Arc};

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
    pub async fn suggest_available_time_2(&self, teacher_id: &str) -> Result<Vec<Schedule>, String> {
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

    // Cambiar a esta versión cuando es sin el facility_id
    pub async fn suggest_available_time(&self, teacher_id: &str) -> Result<Vec<Schedule>, String> {
        let courses = self.course_repo.get_courses_by_user(teacher_id).await?;

        let mut existing_schedules = Vec::new();
        for course in &courses {
            existing_schedules.extend(self.schedule_repo.get_schedules_by_course(&course.id).await?);
        }

        let facility_repo = Arc::new(SupabaseFacilityRepository::new().await?);
        let facilities_available = facility_repo.get_facility_available().await?;
        let mut facilities_by_capacity: BTreeMap<i32, Vec<FacilityAvailable>> = BTreeMap::new();
        for fa in facilities_available {
            facilities_by_capacity.entry(fa.facility.capacity).or_default().push(fa);
        }

        let mut suggested_schedules = Vec::new();
        let days = [Weekday::Monday, Weekday::Tuesday, Weekday::Wednesday, Weekday::Thursday, Weekday::Friday, Weekday::Saturday];

        for course in courses {
            let required_hours = course.hours_per_week;
            let required_capacity = course.capacity;

            let mut facilities_by_capacity_2 = facilities_by_capacity.clone();

            let suitable_facilities: Vec<_> = facilities_by_capacity.range(required_capacity..).flat_map(|(_, facilities)| facilities).collect();

            for facility_available in suitable_facilities {
                let mut schedule_found = false;

                for &day in &days {
                    if let Some(available_slots) = facility_available.available_slots.get(&day) {
                        for &(start, end) in available_slots {
                            let duration = end - start;
                            let available_hours = duration.num_hours() as i32;

                            if available_hours >= required_hours {
                                let new_schedule = Schedule {
                                    id: format!("suggested-{}", suggested_schedules.len() + 1),
                                    course_id: course.id.clone(),
                                    day,
                                    start_time: start,
                                    end_time: start + chrono::Duration::hours(required_hours as i64),
                                    session_type: SessionType::Theory,
                                    location_detail: None,
                                    created_at: None,
                                    facility_id: facility_available.facility.id.clone(),
                                };

                                let has_conflict = existing_schedules.iter().chain(suggested_schedules.iter()).any(|s| s.conflicts_with(&new_schedule));

                                if !has_conflict && self.validate_schedule(teacher_id, &new_schedule).await? {
                                    suggested_schedules.push(new_schedule);
                                    schedule_found = true;

                                    self.update_facility_availability(
                                        &mut facilities_by_capacity_2,
                                        &facility_available.facility.id,
                                        day,
                                        start,
                                        start + chrono::Duration::hours(required_hours as i64),
                                    )
                                    .await?;
                                    break;
                                }
                            }
                        }
                    }
                    if schedule_found {
                        break;
                    }
                }
                if schedule_found {
                    break;
                }
            }
        }

        Ok(suggested_schedules)
    }

    async fn update_facility_availability(
        &self,
        facilities_by_capacity: &mut BTreeMap<i32, Vec<FacilityAvailable>>,
        facility_id: &str,
        day: Weekday,
        used_start: NaiveTime,
        used_end: NaiveTime,
    ) -> Result<(), String> {
        for facilities in facilities_by_capacity.values_mut() {
            for fa in facilities {
                if fa.facility.id == facility_id {
                    if let Some(slots) = fa.available_slots.get_mut(&day) {
                        let mut new_slots = Vec::new();

                        for &(start, end) in slots.iter() {
                            if start < used_start && end > used_end {
                                new_slots.push((start, used_start));
                                new_slots.push((used_end, end));
                            } else if start < used_start && end > used_start {
                                new_slots.push((start, used_start));
                            } else if start < used_end && end > used_end {
                                new_slots.push((used_end, end));
                            }
                        }

                        *slots = new_slots;
                    }
                }
            }
        }

        Ok(())
    }

    pub async fn verify_schedule_by_user(&self, user_id: &str) -> Result<bool, String> {
        let schedules = self.schedule_repo.get_schedules_by_user(user_id).await?;

        Ok(!schedules.is_empty())
    }
}
