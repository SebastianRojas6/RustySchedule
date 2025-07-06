use crate::domain::models::facilitie_available::FacilityAvailable;
use crate::domain::{models::enums::Weekday, models::facilitie::Facility, repositories::facility_repository::FacilityRepository};
use crate::infrastructure::database::entities::{course_schedules, courses, facilities, sea_orm_active_enums::DayType, users};
use async_trait::async_trait;
use chrono::NaiveTime;
use chrono::Utc;
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, JoinType, QueryFilter, QuerySelect, RelationTrait, Set};
use shared::config::connect_to_supabase;
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
        // Get all facilities
        let facilities = facilities::Entity::find().all(&self.db).await.map_err(|e| format!("Failed to fetch facilities: {}", e))?;

        // Get all scheduled courses for facilities
        let schedules = course_schedules::Entity::find()
            .all(&self.db)
            .await
            .map_err(|e| format!("Failed to fetch course schedules: {}", e))?;

        // Group schedules by facility_id and day
        let mut facility_schedules: std::collections::HashMap<String, Vec<course_schedules::Model>> = std::collections::HashMap::new();

        for schedule in schedules {
            facility_schedules.entry(schedule.facility_id.clone()).or_default().push(schedule);
        }

        // Convert to FacilityAvailable models
        let mut result = Vec::new();

        for facility in facilities {
            let facility_id = facility.id.clone();
            let schedules = facility_schedules.get(&facility_id).cloned().unwrap_or_default();

            // Process each day separately
            let days = [
                DayType::Monday,
                DayType::Tuesday,
                DayType::Wednesday,
                DayType::Thursday,
                DayType::Friday,
                DayType::Saturday,
                DayType::Sunday,
            ];

            for day in days.clone() {
                // Filter schedules for this day
                let day_schedules: Vec<_> = schedules.iter().filter(|s| s.day == day).map(|s| (s.start_time, s.end_time)).collect();

                // Sort time ranges by start time
                let mut sorted_ranges = day_schedules.clone();
                sorted_ranges.sort_by(|a, b| a.0.cmp(&b.0));

                // Calculate available slots between scheduled times
                let mut available_start = NaiveTime::from_hms_opt(0, 0, 0).unwrap();

                for (start, end) in sorted_ranges {
                    if available_start < start {
                        result.push(FacilityAvailable {
                            facility: Facility {
                                id: facility.id.clone(),
                                name: facility.name.clone(),
                                capacity: facility.capacity.unwrap_or_default(),
                                facility_type: facility.facility_type.clone().unwrap_or_default(),
                                created_at: Some(Utc::now().naive_utc().to_string()),
                            },
                            weekday: match day {
                                DayType::Monday => Weekday::Monday,
                                DayType::Tuesday => Weekday::Tuesday,
                                DayType::Wednesday => Weekday::Wednesday,
                                DayType::Thursday => Weekday::Thursday,
                                DayType::Friday => Weekday::Friday,
                                DayType::Saturday => Weekday::Saturday,
                                DayType::Sunday => Weekday::Sunday,
                            },
                            start_time: available_start,
                            end_time: start,
                        });
                    }
                    available_start = end;
                }

                // Add remaining time after last schedule
                let day_end = NaiveTime::from_hms_opt(23, 59, 59).unwrap();
                if available_start < day_end {
                    result.push(FacilityAvailable {
                        facility: Facility {
                            id: facility.id.clone(),
                            name: facility.name.clone(),
                            capacity: facility.capacity.unwrap_or_default(),
                            facility_type: facility.facility_type.clone().unwrap_or_default(),
                            created_at: Some(Utc::now().naive_utc().to_string()),
                        },
                        weekday: match day {
                            DayType::Monday => Weekday::Monday,
                            DayType::Tuesday => Weekday::Tuesday,
                            DayType::Wednesday => Weekday::Wednesday,
                            DayType::Thursday => Weekday::Thursday,
                            DayType::Friday => Weekday::Friday,
                            DayType::Saturday => Weekday::Saturday,
                            DayType::Sunday => Weekday::Sunday,
                        },
                        start_time: available_start,
                        end_time: day_end,
                    });
                }
            }

            // If no schedules exist, the facility is available all day every day
            if schedules.is_empty() {
                for day in &days {
                    result.push(FacilityAvailable {
                        facility: Facility {
                            id: facility.id.clone(),
                            name: facility.name.clone(),
                            capacity: facility.capacity.unwrap_or_default(),
                            facility_type: facility.facility_type.clone().unwrap_or_default(),
                            created_at: Some(Utc::now().naive_utc().to_string()),
                        },
                        weekday: match day {
                            DayType::Monday => Weekday::Monday,
                            DayType::Tuesday => Weekday::Tuesday,
                            DayType::Wednesday => Weekday::Wednesday,
                            DayType::Thursday => Weekday::Thursday,
                            DayType::Friday => Weekday::Friday,
                            DayType::Saturday => Weekday::Saturday,
                            DayType::Sunday => Weekday::Sunday,
                        },
                        start_time: NaiveTime::from_hms_opt(0, 0, 0).unwrap(),
                        end_time: NaiveTime::from_hms_opt(23, 59, 59).unwrap(),
                    });
                }
            }
        }

        Ok(result)
    }
}
