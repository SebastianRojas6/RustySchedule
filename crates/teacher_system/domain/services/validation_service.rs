use crate::domain::models::schedule::Schedule;
use crate::domain::repositories::schedule_repository::ScheduleRepository;
use std::sync::Arc;

#[derive(Clone)]
pub struct DefaultValidationService {
    schedule_repo: Arc<dyn ScheduleRepository>,
}

impl DefaultValidationService {
    pub fn new(schedule_repo: Arc<dyn ScheduleRepository>) -> Self {
        Self { schedule_repo }
    }

    pub async fn check_teacher_availability(
        &self,
        teacher_id: &str,
        schedule: &Schedule,
    ) -> Result<bool, String> {
        // Obtener todos los horarios del profesor
        let teacher_schedules = self.schedule_repo.get_schedules_by_user(teacher_id).await?;

        // Verificar conflictos con horarios existentes
        let has_conflict = teacher_schedules.iter().any(|s| s.conflicts_with(schedule));

        // Si hay conflicto, el profesor no está disponible
        Ok(!has_conflict)
    }

    pub async fn check_facility_availability(
        &self,
        facility_id: &str,
        schedule: &Schedule,
    ) -> Result<bool, String> {
        // Obtener todos los horarios de la instalación
        let facility_schedules = self
            .schedule_repo
            .get_schedules_by_facility(facility_id)
            .await?;

        // Verificar conflictos con horarios existentes
        let has_conflict = facility_schedules
            .iter()
            .any(|s| s.conflicts_with(schedule));

        // Si hay conflicto, la instalación no está disponible
        Ok(!has_conflict)
    }
}
