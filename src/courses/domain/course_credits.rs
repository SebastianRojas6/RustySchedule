#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CourseCredits(pub u8);

impl CourseCredits {
    pub fn new(credits: u8) -> Result<Self, String> {
        if credits == 0 || credits > 4 {
            Err("Tiene que estar entre 1 y 4 ".to_string())
        } else {
            Ok(Self(credits))
        }
    }

    pub fn value(&self) -> u8 {
        self.0
    }
}
