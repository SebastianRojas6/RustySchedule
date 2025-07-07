use super::section::SectionId;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct ScheduleSuggestion {
    pub selected_sections: Vec<SectionId>,
}
