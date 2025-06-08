#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CourseCycle(u8);

impl CourseCycle {
    pub fn new(cycle: u8) -> Self {
        assert!(cycle >= 1, "Course cycle must be 1 or greater");
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

    pub fn is_allowed_in_semester(&self, semester_parity: SemesterParity) -> bool {
        match semester_parity {
            SemesterParity::Even => self.is_even(),
            SemesterParity::Odd => self.is_odd(),
        }
    }
}
