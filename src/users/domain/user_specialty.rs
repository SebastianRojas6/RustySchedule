#[derive(Debug, Clone)]
pub struct Specialty(String);

impl Specialty {
    pub fn new(name: String) -> Result<Self, String> {
        if !name.is_empty() {
            Ok(Self(name))
        } else {
            Err("Nombre de especialidad vacío".to_string())
        }
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}
