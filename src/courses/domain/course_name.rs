#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CourseName(pub String);

impl CourseName {
    pub fn new(name: String) -> Result<Self, String> {
        if name.trim().is_empty() {
            Err("Bro, el curso no puede ser vacío".to_string())
        } else {
            Ok(Self(name))
        }
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}
