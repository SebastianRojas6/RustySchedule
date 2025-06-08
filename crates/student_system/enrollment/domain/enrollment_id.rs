#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct EnrollmentId(String);

impl EnrollmentId {
    pub fn new(id: String) -> Self {
        assert!(!id.trim().is_empty(), "Bro, no puede ser vacío");
        Self(id)
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}
