#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CourseCode(pub String);

impl CourseCode {
    pub fn new(code: String) -> Result<Self, String> {
        if code.trim().is_empty() {
            return Err("No puede estar vacío".to_string());
        }

        if !code.chars().all(|c| c.is_alphanumeric()) {
            return Err("ponlo bien xdd".to_string());
        }
        Ok(Self(code))
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}