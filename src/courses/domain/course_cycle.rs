#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CourseCycle(pub u8);

impl CourseCycle {
    pub fn new(value: u8) -> Result<Self, String> {
        if value >= 1 && value <= 10 {
            Ok(Self(value))
        } else {
            Err("El ciclo debe estar entre 1 y 10".to_string())
        }
    }

    pub fn is_valid(&self) -> bool {
        self.0 >= 1 && self.0 <= 10 
    }

    pub fn is_odd(&self) -> bool {
        self.0 % 2 == 1
    }
}
