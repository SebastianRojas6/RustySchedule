#[derive(Debug, Clone)]
pub struct UserCode(String);

impl UserCode {
    pub fn new(code: String) -> Result<Self, String> {
        if code.len() == 8 && code.chars().all(|c| c.is_digit(10)) {
            Ok(Self(code))
        } else {
            Err("Código de usuario inválido".to_string())
        }
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}
