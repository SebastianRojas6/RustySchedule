#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CourseCredits(pub i32);

impl CourseCredits {
    pub fn new(credits: i32) -> Result<Self, String> {
        if credits == 0 || credits > 4 {
            Err("Tiene que estar entre 1 y 4 ".to_string())
        } else {
            Ok(Self(credits))
        }
    }

    pub fn value(&self) -> i32 {
        self.0
    }
}
