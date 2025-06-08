#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct EnrollmentId(pub String);

impl EnrollmentId {
    pub fn new(id: String) -> Self {
        Self(id)
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}
