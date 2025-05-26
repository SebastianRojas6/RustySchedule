mod courses;

use courses::domain::{
    Course, CourseName, CourseSection, CourseCurriculum,
    CourseCapacity, CourseCycle, CourseCode, CourseCredits,
};

fn main() {


    let course = Course {
        name: CourseName::new("Cálculo I".to_string()).unwrap(),
        code: CourseCode("MAT101".to_string()),
        section: CourseSection::try_from(1).unwrap(),
        curriculum: CourseCurriculum::M2023,
        capacity: CourseCapacity(45),
        prerequisites: vec!["math001".to_string()],
        teacher_id: "teacher-123".to_string(),
        facility_id: "room-01".to_string(),
        cycle: CourseCycle::new(1).unwrap(),
        credits: CourseCredits::new(4).unwrap(),
    };

    debug_course(&course);

}


fn debug_course(course: &Course) {
    println!("Code: {:?}", course.code);
    println!("Name: {:?}", course.name);
    println!("Section: {:?}", course.section);
    println!("Curriculum: {:?}", course.curriculum);
    println!("Capacity: {:?}", course.capacity);
    println!("Credits: {:?}", course.credits);
    println!("Prerequisites: {:?}", course.prerequisites);
    println!("Teacher ID: {:?}", course.teacher_id);
    println!("Facility ID: {:?}", course.facility_id);
    println!("Cycle: {:?}", course.cycle);
}