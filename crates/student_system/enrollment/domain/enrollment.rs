use super::enrollment_id::EnrollmentId;
use super::user_id::UserId;
use super::CourseId;
use super::enrollment_status::EnrollmentStatus;

use super::CreditAmount;
use super::StudentStatus;
use super::CourseCycle;
use super::SemesterParity;
use super::error::EnrollmentError;

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, PartialEq, Eq , Serialize, Deserialize)]
pub struct Enrollment {
    pub id: EnrollmentId,
    pub student_id: UserId,
    pub course_id: CourseId,
    pub status: EnrollmentStatus,
}

impl Enrollment {
    #[allow(clippy::too_many_arguments)]
    pub fn try_enroll(
        id: EnrollmentId,
        student_id: UserId,
        course_id: CourseId,
        student_status: StudentStatus,
        student_credits_enrolled: CreditAmount,
        course_credits: CreditAmount,
        completed_courses: &[CourseId],
        course_prerequisites: &[CourseId],
        course_cycle: CourseCycle,
        semester: SemesterParity,
        section_capacity_available: bool,
        already_enrolled: bool,
        times_repeated: u8,
        schedule_conflict: bool,
        course_already_passed: bool,
    ) -> Result<Self, EnrollmentError> {
        if already_enrolled {
            return Err(EnrollmentError::AlreadyEnrolled);
        }

        if course_already_passed {
            return Err(EnrollmentError::CourseAlreadyPassed);
        }

        if !course_prerequisites.iter().all(|p| completed_courses.contains(p)) {
            return Err(EnrollmentError::MissingPrerequisites);
        }

        let total_credits = student_credits_enrolled + course_credits;

        match student_status {
            StudentStatus::Observed if total_credits.value() > 14 => {
                return Err(EnrollmentError::ExceedsObservedLimit);
            }
            StudentStatus::Regular if total_credits.value() > 26 => {
                return Err(EnrollmentError::ExceedsCreditLimit);
            }
            _ => {}
        }

        if !section_capacity_available {
            return Err(EnrollmentError::SectionFull);
        }

        if schedule_conflict {
            return Err(EnrollmentError::ScheduleConflict);
        }

        if (semester == SemesterParity::Even && course_cycle.is_odd())
            || (semester == SemesterParity::Odd && course_cycle.is_even())
        {
            return Err(EnrollmentError::InvalidCycleForSemester);
        }

        if times_repeated >= 3 {
            return Err(EnrollmentError::CourseRepeatLimitExceeded);
        }

        Ok(Self {
            id,
            student_id,
            course_id,
            status: EnrollmentStatus::Enrolled,
        })
    }

    pub fn drop(&mut self) {
        self.status = EnrollmentStatus::Dropped;
    }

    pub fn complete(&mut self) -> Result<(), EnrollmentError> {
    match self.status {
        EnrollmentStatus::Enrolled => {
            self.status = EnrollmentStatus::Completed;
            Ok(())
        }
        _ => Err(EnrollmentError::InvalidCompletionState(self.status.clone())),
    }
}

}
