// crates/student_system/suggestion/infrastructure/repository.rs

use sea_orm::prelude::*;
use async_trait::async_trait;
use crate::enrollment::infrastructure::entity::{availabilities, users};
use crate::suggestion::domain::repository::SuggestionRepository;
use crate::suggestion::domain::section::{Day, TimeSlot};

pub struct SupabaseSuggestionRepository {
    pub db: DatabaseConnection,
}

#[async_trait]
impl SuggestionRepository for SupabaseSuggestionRepository {
    async fn find_student_status(&self, student_id: &str) -> Option<String> {
        users::Entity::find_by_id(student_id)
            .one(&self.db)
            .await
            .ok()
            .flatten()
            .and_then(|u| u.student_status.map(|s| s.to_string()))
    }

    async fn find_availabilities(&self, student_id: &str) -> Vec<TimeSlot> {
        let result = availabilities::Entity::find()
            .filter(availabilities::Column::StudentId.eq(student_id))
            .all(&self.db)
            .await;

        match result {
            Ok(rows) => rows
                .into_iter()
                .filter_map(|row| {
                    let day = match row.day.to_string().as_str() {
                        "Monday" => Day::Monday,
                        "Tuesday" => Day::Tuesday,
                        "Wednesday" => Day::Wednesday,
                        "Thursday" => Day::Thursday,
                        "Friday" => Day::Friday,
                        "Saturday" => Day::Saturday,
                        "Sunday" => Day::Sunday,
                        _ => return None,
                    };

                    Some(TimeSlot {
                        day,
                        start_time: row.start_time,
                        end_time: row.end_time,
                    })
                })
                .collect(),
            Err(_) => vec![],
        }
    }
}
