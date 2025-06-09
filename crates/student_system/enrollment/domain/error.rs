use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum EnrollmentError {
    
    #[error("Already enrolled")]
    AlreadyEnrolled,
    
    #[error("Curriculum mismatch")]
    CurriculumMismatch,
    
    #[error("Course already passed")]
    CourseAlreadyPassed,
    
    #[error("Missing prerequisites")]
    MissingPrerequisites,
    
    #[error("Exceeds credit limit")]
    ExceedsCreditLimit,
    
    #[error("Exceeds observed limit")]
    ExceedsObservedLimit,
    
    #[error("Section full")]
    SectionFull,
    
    #[error("Schedule conflict")]
    ScheduleConflict,
    
    #[error("Invalid cycle for semester")]
    InvalidCycleForSemester,
    
    #[error("Course repeat limit exceeded")]
    CourseRepeatLimitExceeded,
}
