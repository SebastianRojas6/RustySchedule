#[derive(Debug, Clone)]
pub struct Faculty(String);

impl Faculty {
    pub fn new(name: String) -> Result<Self, String> {
        if !name.is_empty() {
            Ok(Self(name))
        } else {
            Err("Nombre de facultad vacío".to_string())
        }
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}
