#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CourseCycle(u8);

impl CourseCycle {
    pub fn new(cycle: u8) -> Self {
        assert!(cycle >= 1, "Cycle must be 1 or greater");
        Self(cycle)
    }

    pub fn value(&self) -> u8 {
        self.0
    }

    pub fn is_even(&self) -> bool {
        self.0 % 2 == 0
    }

    pub fn is_odd(&self) -> bool {
        self.0 % 2 == 1
    }
}
