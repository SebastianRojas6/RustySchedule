#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CourseCapacity(pub i32);

impl CourseCapacity {
    const MAX_PER_SECTION: i32 = 45;

        pub fn new(value: i32) -> Result<Self, String> {
        if value > 0 {
            Ok(Self(value))
        } else {
            Err("La capacidad debe ser mayor a 0".to_string())
        }
    }

    pub fn is_valid(&self) -> bool {
        self.0 > 0
    }

    pub fn total_students(&self) -> i32 {
        self.0
    }

    pub fn calculate_sections(&self, cycle: u8) -> u8 {
        if cycle % 2 == 1 {
            if self.0 > Self::MAX_PER_SECTION {
                ((self.0 + Self::MAX_PER_SECTION - 1) / Self::MAX_PER_SECTION) as u8
            } else {
                1
            }
        } else {
            ((self.0 + Self::MAX_PER_SECTION - 1) / Self::MAX_PER_SECTION) as u8
        }
    }

    pub fn value(&self) -> i32 {
        self.0
    }
}
