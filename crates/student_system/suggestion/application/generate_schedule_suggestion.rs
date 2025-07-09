use crate::enrollment::application::find_available_courses_to_enroll::FindAvailableCoursesToEnrollUseCase;
use crate::enrollment::application::find_available_sections_by_course_code::FindAvailableSectionsByCourseCodeUseCase;
use crate::enrollment::domain::user_id::UserId;
use crate::suggestion::domain::repository::SuggestionRepository;
use crate::suggestion::domain::section::{Day, SectionId, TimeSlot};
use crate::suggestion::domain::suggestion::ScheduleSuggestion;
use chrono::NaiveTime;

pub struct GenerateScheduleSuggestionUseCase<'a> {
    pub suggestion_repo: &'a dyn SuggestionRepository,
    pub enrollment_use_case: &'a FindAvailableCoursesToEnrollUseCase<'a>,
    pub section_use_case: &'a FindAvailableSectionsByCourseCodeUseCase<'a>,
}

impl<'a> GenerateScheduleSuggestionUseCase<'a> {
    pub fn new(
        suggestion_repo: &'a dyn SuggestionRepository,
        enrollment_use_case: &'a FindAvailableCoursesToEnrollUseCase<'a>,
        section_use_case: &'a FindAvailableSectionsByCourseCodeUseCase<'a>,
    ) -> Self {
        Self {
            suggestion_repo,
            enrollment_use_case,
            section_use_case,
        }
    }

    pub async fn execute(&self, student_id: &str) -> Result<ScheduleSuggestion, String> {
        let student_status = self
            .suggestion_repo
            .find_student_status(student_id)
            .await
            .unwrap_or("regular".to_string());

        let has_availabilities = !self
            .suggestion_repo
            .find_availabilities(student_id)
            .await
            .is_empty();

        let max_credits = if has_availabilities {
            i32::MAX
        } else if student_status == "regular" {
            26
        } else {
            0 
        };

        let user_id = UserId::new(student_id.to_string());
        let available_courses = self.enrollment_use_case.execute(&user_id).await?;

        let mut selected_sections = Vec::new();
        let mut scheduled_times = Vec::new();
        let mut total_credits = 0;

        for course in available_courses {
            if total_credits + course.credits > max_credits {
                continue;
            }

            let course_sections = self
            .section_use_case
            .execute(&course.code)
            .await?;

            for section in &course_sections.secciones {
                let section_times = vec![TimeSlot {
                    day: match section.day.to_string().as_str() {
                        "monday" => Day::Monday,
                        "tuesday" => Day::Tuesday,
                        "wednesday" => Day::Wednesday,
                        "thursday" => Day::Thursday,
                        "friday" => Day::Friday,
                        "saturday" => Day::Saturday,
                        "sunday" => Day::Sunday,
                        _ => continue,
                    },
                    start_time: NaiveTime::parse_from_str(&section.start_time, "%H:%M:%S")
                        .unwrap_or(NaiveTime::from_hms_opt(0, 0, 0).unwrap()),
                    end_time: NaiveTime::parse_from_str(&section.end_time, "%H:%M:%S")
                        .unwrap_or(NaiveTime::from_hms_opt(0, 0, 0).unwrap()),
                }];

                let conflicts = section_times.iter().any(|slot| {
                    scheduled_times.iter().any(|s: &TimeSlot| s.conflicts_with(slot))
                });

                if conflicts {
                    continue;
                }

                scheduled_times.extend(section_times);
                selected_sections.push(SectionId(course.id.clone()));
                total_credits += course.credits;
                break;
            }
        }

        Ok(ScheduleSuggestion {
            selected_sections,
        })
    }
}
