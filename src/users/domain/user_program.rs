#[derive(Debug, Clone)]
pub struct Program(String);

impl Program {
    pub fn new(name: String) -> Result<Self, String> {
        if !name.is_empty() {
            Ok(Self(name))
        } else {
            Err("Nombre de programa vacío".to_string())
        }
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}
