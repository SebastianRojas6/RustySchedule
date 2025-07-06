use crate::domain::models::facilitie_available::FacilityAvailable;
use crate::domain::{models::enums::Weekday, models::facilitie::Facility, repositories::facility_repository::FacilityRepository};
use crate::infrastructure::database::entities::{course_schedules, courses, facilities, sea_orm_active_enums::DayType, users};
use async_trait::async_trait;
use chrono::NaiveTime;
use chrono::Utc;
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, JoinType, QueryFilter, QuerySelect, RelationTrait, Set};
use shared::config::connect_to_supabase;
use std::collections::BTreeMap;
#[derive(Clone)]
pub struct SupabaseFacilityRepository {
    db: DatabaseConnection,
}

impl SupabaseFacilityRepository {
    pub async fn new() -> Result<Self, String> {
        let db = connect_to_supabase().await.map_err(|e| e.to_string())?;
        Ok(Self { db })
    }
}

#[async_trait]
impl FacilityRepository for SupabaseFacilityRepository {
    async fn create_facility(&self, facility: &Facility) -> Result<(), String> {
        let new_facility = facilities::ActiveModel {
            id: Set(facility.id.clone()),
            name: Set(facility.name.clone()),
            capacity: Set(Some(facility.capacity.clone())),
            facility_type: Set(Some(facility.facility_type.clone())),
            created_at: Set(Some(Utc::now().naive_utc())),
        };

        new_facility.insert(&self.db).await.map_err(|e| e.to_string())?;
        Ok(())
    }

    async fn update_facility(&self, facility: &Facility) -> Result<(), String> {
        let existing = facilities::Entity::find_by_id(&facility.id)
            .one(&self.db)
            .await
            .map_err(|e| e.to_string())?
            .ok_or_else(|| "Facility not found".to_string())?;

        let mut model: facilities::ActiveModel = existing.into();
        model.name = Set(facility.name.clone());
        model.capacity = Set(Some(facility.capacity.clone()));
        model.facility_type = Set(Some(facility.facility_type.clone()));

        model.update(&self.db).await.map_err(|e| e.to_string())?;
        Ok(())
    }

    async fn get_facility_by_id(&self, id: &str) -> Result<Facility, String> {
        facilities::Entity::find_by_id(id)
            .one(&self.db)
            .await
            .map_err(|e| e.to_string())?
            .map(|model| Facility {
                id: model.id,
                name: model.name,
                capacity: model.capacity.unwrap_or_default(),
                facility_type: model.facility_type.unwrap_or_default(),
                created_at: model.created_at.map(|dt| dt.to_string()),
            })
            .ok_or_else(|| "Facility not found".to_string())
    }

    async fn get_all_facilities(&self) -> Result<Vec<Facility>, String> {
        facilities::Entity::find()
            .all(&self.db)
            .await
            .map_err(|e| e.to_string())?
            .into_iter()
            .map(|model| {
                Ok(Facility {
                    id: model.id,
                    name: model.name,
                    capacity: model.capacity.unwrap_or_default(),
                    facility_type: model.facility_type.unwrap_or_default(),
                    created_at: model.created_at.map(|dt| dt.to_string()),
                })
            })
            .collect()
    }

    async fn get_facilities_by_course(&self, course_id: &str) -> Result<Facility, String> {
        let schedules = course_schedules::Entity::find()
            .filter(course_schedules::Column::CourseId.eq(course_id))
            .find_also_related(facilities::Entity)
            .one(&self.db)
            .await
            .map_err(|e| e.to_string())?;

        let facilities = schedules
            .into_iter()
            .filter_map(|(_, facility_opt)| facility_opt)
            .next()
            .map(|model| Facility {
                id: model.id,
                name: model.name,
                capacity: model.capacity.unwrap_or_default(),
                facility_type: model.facility_type.unwrap_or_default(),
                created_at: model.created_at.map(|dt| dt.to_string()),
            })
            .ok_or_else(|| "No facility found for this course".to_string())?;

        Ok(facilities)
    }

    async fn get_facilities_name_course(&self, name_course: &str) -> Result<Vec<Facility>, String> {
        let facilities = facilities::Entity::find()
            .join(JoinType::InnerJoin, facilities::Relation::CourseSchedules.def())
            .join(JoinType::InnerJoin, course_schedules::Relation::Courses.def())
            .filter(courses::Column::Name.eq(name_course))
            .all(&self.db)
            .await
            .map_err(|e| e.to_string())?
            .into_iter()
            .map(|model| Facility {
                id: model.id,
                name: model.name,
                capacity: model.capacity.unwrap_or_default(),
                facility_type: model.facility_type.unwrap_or_default(),
                created_at: model.created_at.map(|dt| dt.to_string()),
            })
            .collect();

        Ok(facilities)
    }

    async fn get_facilities_by_schedule(&self, schedule_id: &str) -> Result<Facility, String> {
        course_schedules::Entity::find_by_id(schedule_id)
            .find_also_related(facilities::Entity)
            .one(&self.db)
            .await
            .map_err(|e| e.to_string())?
            .and_then(|(_, facility)| facility)
            .map(|model| Facility {
                id: model.id,
                name: model.name,
                capacity: model.capacity.unwrap_or_default(),
                facility_type: model.facility_type.unwrap_or_default(),
                created_at: model.created_at.map(|dt| dt.to_string()),
            })
            .ok_or_else(|| "Facility not found for this schedule".to_string())
    }

    async fn get_facilities_by_user(&self, user_id: &str) -> Result<Vec<Facility>, String> {
        let facilitie = facilities::Entity::find()
            .join(JoinType::InnerJoin, facilities::Relation::CourseSchedules.def())
            .join(JoinType::InnerJoin, courses::Relation::Users.def())
            .filter(users::Column::Id.eq(user_id))
            .all(&self.db)
            .await
            .map_err(|e| e.to_string())?
            .into_iter()
            .map(|model| Facility {
                id: model.id,
                name: model.name,
                capacity: model.capacity.unwrap_or_default(),
                facility_type: model.facility_type.unwrap_or_default(),
                created_at: model.created_at.map(|dt| dt.to_string()),
            })
            .collect();

        Ok(facilitie)
    }

    async fn delete_facility(&self, id: &str) -> Result<(), String> {
        facilities::Entity::delete_by_id(id).exec(&self.db).await.map_err(|e| e.to_string())?;
        Ok(())
    }

    async fn get_facility_available(&self) -> Result<Vec<FacilityAvailable>, String> {
        let facilities = self.get_all_facilities().await?;

        let occupied_schedules = course_schedules::Entity::find()
            .all(&self.db)
            .await
            .map_err(|e| e.to_string())?
            .into_iter()
            .fold(BTreeMap::new(), |mut acc, schedule| {
                acc.entry(schedule.facility_id.clone()).or_insert_with(Vec::new).push(schedule);
                acc
            });

        let workday_start = NaiveTime::from_hms_opt(8, 0, 0).unwrap();
        let workday_end = NaiveTime::from_hms_opt(22, 0, 0).unwrap();
        let days = vec![Weekday::Monday, Weekday::Tuesday, Weekday::Wednesday, Weekday::Thursday, Weekday::Friday, Weekday::Saturday];

        let mut result = Vec::new();

        for facility in facilities {
            let facility_id = facility.id.clone();
            let occupied = occupied_schedules.get(&facility_id).cloned().unwrap_or_default();

            let mut occupied_by_day: BTreeMap<Weekday, Vec<(NaiveTime, NaiveTime)>> = BTreeMap::new();

            for schedule in occupied {
                let day = match schedule.day {
                    DayType::Monday => Weekday::Monday,
                    DayType::Tuesday => Weekday::Tuesday,
                    DayType::Wednesday => Weekday::Wednesday,
                    DayType::Thursday => Weekday::Thursday,
                    DayType::Friday => Weekday::Friday,
                    DayType::Saturday => Weekday::Saturday,
                    DayType::Sunday => Weekday::Sunday,
                };

                occupied_by_day.entry(day).or_default().push((schedule.start_time, schedule.end_time));
            }

            let mut available_slots = BTreeMap::new();

            for &day in &days {
                let mut day_slots = Vec::new();
                let mut current_time = workday_start;

                let mut day_occupied = occupied_by_day.get(&day).cloned().unwrap_or_default();
                day_occupied.sort_by(|a, b| a.0.cmp(&b.0));

                for (start, end) in day_occupied {
                    if start > current_time {
                        day_slots.push((current_time, start));
                    }
                    current_time = current_time.max(end);
                }

                if current_time < workday_end {
                    day_slots.push((current_time, workday_end));
                }

                if !day_slots.is_empty() {
                    available_slots.insert(day, day_slots);
                }
            }

            result.push(FacilityAvailable {
                facility: Facility {
                    id: facility.id,
                    name: facility.name,
                    capacity: facility.capacity,
                    facility_type: facility.facility_type,
                    created_at: facility.created_at.map(|dt| dt.to_string()),
                },
                available_slots,
            });
        }

        Ok(result)
    }
}
