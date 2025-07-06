use crate::domain::{models::enums::Weekday, models::schedule::Schedule, repositories::schedule_repository::ScheduleRepository};
use crate::infrastructure::database::entities::{course_schedules, courses, facilities, sea_orm_active_enums};
use async_trait::async_trait;
use chrono::Utc;
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, JoinType, QueryFilter, QuerySelect, RelationTrait, Set};
use shared::config::connect_to_supabase;
use uuid::Uuid;

#[derive(Clone)]
pub struct SupabaseScheduleRepository {
    db: DatabaseConnection,
}

impl SupabaseScheduleRepository {
    pub async fn new() -> Result<Self, String> {
        let db = connect_to_supabase().await.map_err(|e| e.to_string())?;
        Ok(Self { db })
    }
}

#[async_trait]
impl ScheduleRepository for SupabaseScheduleRepository {
    async fn create_schedule(&self, schedule: &Schedule) -> Result<Schedule, String> {
        let schedule_id = if schedule.id.is_empty() { Uuid::new_v4().to_string() } else { schedule.id.clone() };

        let day_db = sea_orm_active_enums::to_db_daytype(&schedule.day);
        let session_type_db = sea_orm_active_enums::to_db_session(&schedule.session_type);

        let schedule_model = course_schedules::ActiveModel {
            id: Set(schedule_id.clone()),
            course_id: Set(schedule.course_id.clone()),
            day: Set(day_db),
            start_time: Set(schedule.start_time),
            end_time: Set(schedule.end_time),
            session_type: Set(session_type_db),
            location_detail: Set(schedule.location_detail.clone()),
            facility_id: Set(schedule.facility_id.clone()),
            created_at: Set(Some(Utc::now().naive_utc())),
            ..Default::default()
        };

        let result = schedule_model.insert(&self.db).await.map_err(|e| e.to_string())?;

        let created_schedule = Schedule {
            id: result.id,
            course_id: result.course_id,
            day: schedule.day.clone(),
            start_time: result.start_time,
            end_time: result.end_time,
            session_type: schedule.session_type.clone(),
            location_detail: result.location_detail,
            created_at: result.created_at.map(|dt| dt.to_string()),
            facility_id: result.facility_id,
        };

        Ok(created_schedule)
    }

    async fn update_schedule(&self, schedule: &Schedule) -> Result<Schedule, String> {
        dbg!(&schedule);
        let mut schedule_model = match course_schedules::Entity::find_by_id(&schedule.id).one(&self.db).await.map_err(|e| e.to_string())? {
            Some(existing) => existing.into(),
            None => {
                let new_id = if schedule.id.is_empty() { Uuid::new_v4().to_string() } else { schedule.id.clone() };

                course_schedules::ActiveModel {
                    id: Set(new_id),
                    ..Default::default()
                }
            }
        };

        schedule_model.course_id = Set(schedule.course_id.clone());
        schedule_model.day = Set(sea_orm_active_enums::to_db_daytype(&schedule.day));
        schedule_model.start_time = Set(schedule.start_time);
        schedule_model.end_time = Set(schedule.end_time);
        schedule_model.session_type = Set(sea_orm_active_enums::to_db_session(&schedule.session_type));
        schedule_model.location_detail = Set(schedule.location_detail.clone());
        schedule_model.facility_id = Set(schedule.facility_id.clone());

        let updated_schedule = schedule_model.update(&self.db).await.map_err(|e| e.to_string())?;

        dbg!(&updated_schedule);
        Ok(Schedule {
            id: updated_schedule.id,
            course_id: updated_schedule.course_id,
            day: schedule.day.clone(),
            start_time: updated_schedule.start_time,
            end_time: updated_schedule.end_time,
            session_type: schedule.session_type.clone(),
            location_detail: updated_schedule.location_detail,
            created_at: updated_schedule.created_at.map(|dt| dt.to_string()),
            facility_id: updated_schedule.facility_id,
        })
    }

    async fn get_schedule_by_id(&self, schedule_id: &str) -> Result<Option<Schedule>, String> {
        let course_schedule = course_schedules::Entity::find_by_id(schedule_id)
            .one(&self.db)
            .await
            .map_err(|e| e.to_string())?
            .map(|s| Schedule {
                id: s.id,
                course_id: s.course_id,
                day: sea_orm_active_enums::to_domain_weekday(&s.day),
                start_time: s.start_time,
                end_time: s.end_time,
                session_type: sea_orm_active_enums::to_domain_session(&s.session_type),
                location_detail: s.location_detail,
                created_at: s.created_at.map(|dt| dt.to_string()),
                facility_id: s.facility_id,
            });

        Ok(course_schedule)
    }

    async fn get_all_schedules(&self) -> Result<Vec<Schedule>, String> {
        let schedules = course_schedules::Entity::find()
            .all(&self.db)
            .await
            .map_err(|e| e.to_string())?
            .into_iter()
            .map(|s| Schedule {
                id: s.id,
                course_id: s.course_id,
                day: sea_orm_active_enums::to_domain_weekday(&s.day),
                start_time: s.start_time,
                end_time: s.end_time,
                session_type: sea_orm_active_enums::to_domain_session(&s.session_type),
                location_detail: s.location_detail,
                created_at: s.created_at.map(|dt| dt.to_string()),
                facility_id: s.facility_id,
            })
            .collect();

        Ok(schedules)
    }

    async fn get_schedules_by_course(&self, course_id: &str) -> Result<Option<Schedule>, String> {
        let schedules = course_schedules::Entity::find()
            .filter(course_schedules::Column::CourseId.eq(course_id))
            .all(&self.db)
            .await
            .map_err(|e| e.to_string())?
            .into_iter()
            .map(|s| Schedule {
                id: s.id,
                course_id: s.course_id,
                day: sea_orm_active_enums::to_domain_weekday(&s.day),
                start_time: s.start_time,
                end_time: s.end_time,
                session_type: sea_orm_active_enums::to_domain_session(&s.session_type),
                location_detail: s.location_detail,
                created_at: s.created_at.map(|dt| dt.to_string()),
                facility_id: s.facility_id,
            })
            .next();

        Ok(schedules)
    }

    async fn get_schedules_by_course_name(&self, name_course: &str) -> Result<Vec<Schedule>, String> {
        let schedules = course_schedules::Entity::find()
            .join(JoinType::InnerJoin, course_schedules::Relation::Courses.def())
            .filter(courses::Column::Name.eq(name_course))
            .all(&self.db)
            .await
            .map_err(|e| e.to_string())?
            .into_iter()
            .map(|s| Schedule {
                id: s.id,
                course_id: s.course_id,
                day: sea_orm_active_enums::to_domain_weekday(&s.day),
                start_time: s.start_time,
                end_time: s.end_time,
                session_type: sea_orm_active_enums::to_domain_session(&s.session_type),
                location_detail: s.location_detail,
                created_at: s.created_at.map(|dt| dt.to_string()),
                facility_id: s.facility_id,
            })
            .collect();

        Ok(schedules)
    }

    async fn get_schedules_by_facility(&self, facility_id: &str) -> Result<Vec<Schedule>, String> {
        let schedules = course_schedules::Entity::find()
            .filter(course_schedules::Column::FacilityId.eq(facility_id))
            .all(&self.db)
            .await
            .map_err(|e| e.to_string())?
            .into_iter()
            .map(|s| Schedule {
                id: s.id,
                course_id: s.course_id,
                day: sea_orm_active_enums::to_domain_weekday(&s.day),
                start_time: s.start_time,
                end_time: s.end_time,
                session_type: sea_orm_active_enums::to_domain_session(&s.session_type),
                location_detail: s.location_detail,
                created_at: s.created_at.map(|dt| dt.to_string()),
                facility_id: s.facility_id,
            })
            .collect();

        Ok(schedules)
    }

    async fn get_schedules_by_facility_name(&self, name_facility: &str) -> Result<Vec<Schedule>, String> {
        let schedules = course_schedules::Entity::find()
            .join(JoinType::InnerJoin, course_schedules::Relation::Facilities.def())
            .filter(facilities::Column::Name.eq(name_facility))
            .all(&self.db)
            .await
            .map_err(|e| e.to_string())?
            .into_iter()
            .map(|s| Schedule {
                id: s.id,
                course_id: s.course_id,
                day: sea_orm_active_enums::to_domain_weekday(&s.day),
                start_time: s.start_time,
                end_time: s.end_time,
                session_type: sea_orm_active_enums::to_domain_session(&s.session_type),
                location_detail: s.location_detail,
                created_at: s.created_at.map(|dt| dt.to_string()),
                facility_id: s.facility_id,
            })
            .collect();

        Ok(schedules)
    }

    async fn get_schedules_by_user(&self, user_id: &str) -> Result<Vec<Schedule>, String> {
        let schedules = course_schedules::Entity::find()
            .join(JoinType::InnerJoin, course_schedules::Relation::Courses.def())
            .filter(courses::Column::TeacherId.eq(user_id))
            .all(&self.db)
            .await
            .map_err(|e| e.to_string())?
            .into_iter()
            .map(|s| Schedule {
                id: s.id,
                course_id: s.course_id,
                day: sea_orm_active_enums::to_domain_weekday(&s.day),
                start_time: s.start_time,
                end_time: s.end_time,
                session_type: sea_orm_active_enums::to_domain_session(&s.session_type),
                location_detail: s.location_detail,
                created_at: s.created_at.map(|dt| dt.to_string()),
                facility_id: s.facility_id,
            })
            .collect();

        Ok(schedules)
    }

    async fn get_schedules_by_weekday(&self, weekday: &str) -> Result<Vec<Schedule>, String> {
        let day_enum = sea_orm_active_enums::to_db_daytype(&Weekday::from_str(weekday).unwrap());

        let schedule = course_schedules::Entity::find()
            .filter(course_schedules::Column::Day.eq(day_enum))
            .all(&self.db)
            .await
            .map_err(|e| e.to_string())?
            .into_iter()
            .map(|s| Schedule {
                id: s.id,
                course_id: s.course_id,
                day: sea_orm_active_enums::to_domain_weekday(&s.day),
                start_time: s.start_time,
                end_time: s.end_time,
                session_type: sea_orm_active_enums::to_domain_session(&s.session_type),
                location_detail: s.location_detail,
                created_at: s.created_at.map(|dt| dt.to_string()),
                facility_id: s.facility_id,
            })
            .collect();

        Ok(schedule)
    }

    async fn delete_schedule(&self, schedule_id: &str) -> Result<(), String> {
        course_schedules::Entity::delete_by_id(schedule_id).exec(&self.db).await.map_err(|e| e.to_string())?;

        Ok(())
    }
}
