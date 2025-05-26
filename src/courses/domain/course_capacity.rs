#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CourseCapacity(pub u32);

impl CourseCapacity {
    const MAX_PER_SECTION: u32 = 45;

    pub fn is_valid(&self) -> bool {
        self.0 > 0
    }

    pub fn total_students(&self) -> u32 {
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
}
