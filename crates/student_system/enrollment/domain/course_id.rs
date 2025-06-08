#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CourseId(pub String);

impl CourseId {
    pub fn new(id: String) -> Self {
        Self(id)
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}
