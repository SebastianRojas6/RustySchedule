#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CourseCycle(pub i32);

impl CourseCycle {
    pub fn new(value: i32) -> Result<Self, String> {
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

    //value
    pub fn value(&self) -> i32 {
        self.0
    }
}
