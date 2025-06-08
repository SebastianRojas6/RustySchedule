#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CurriculumId(pub String);

impl CurriculumId {
    pub fn new(id: String) -> Self {
        Self(id)
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}
