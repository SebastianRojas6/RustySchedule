use std::ops::Add;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CreditAmount(u8);

impl CreditAmount {
    pub const MAX_CREDITS_REGULAR: u8 = 26;
    pub const MAX_CREDITS_OBSERVED: u8 = 14;
    pub const MIN_CREDITS: u8 = 1;

    pub fn new(value: u8) -> Self {
        Self(value)
    }

    pub fn value(&self) -> u8 {
        self.0
    }

    pub fn is_valid_for_regular(&self) -> bool {
        self.0 >= Self::MIN_CREDITS && self.0 <= Self::MAX_CREDITS_REGULAR
    }

    pub fn is_valid_for_observed(&self) -> bool {
        self.0 <= Self::MAX_CREDITS_OBSERVED
    }

    pub fn exceeds(&self, limit: CreditAmount) -> bool {
        self.0 > limit.0
    }
}

impl Add for CreditAmount {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self(self.0 + other.0)
    }
}
