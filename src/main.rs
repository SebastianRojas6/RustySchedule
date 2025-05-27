mod courses;

use courses::domain::{
    course::Course, 
    course_name::CourseName, 
    course_section::CourseSection, 
    course_curriculum::CourseCurriculum,
    course_capacity::CourseCapacity, 
    course_cycle::CourseCycle, 
    course_code::CourseCode, 
    course_credits::CourseCredits,
};


mod users;

use users::domain::{
    user::User,
    user_code::UserCode,
    user_faculty::Faculty,
    user_program::Program,
    user_specialty::Specialty,
};


fn main() {


    let course = Course {
        name: CourseName::new("Cálculo II".to_string()).unwrap(),
        code: CourseCode::new("INO204".to_string()).unwrap(),        
        section: CourseSection::try_from(1).unwrap(),
        curriculum: CourseCurriculum::M2023,
        capacity: CourseCapacity::new(45).unwrap(),
        prerequisites: vec!["Cálculo I".to_string()],
        teacher_id: "Hellen-Deidad".to_string(),
        facility_id: "NP-103".to_string(),
        cycle: CourseCycle::new(1).unwrap(),
        credits: CourseCredits::new(4).unwrap(),
        enrolled: 0,
        schedule: vec![],
    };

     let user = User {
        code: UserCode::new("22200261".to_string()).unwrap(),
        faculty: Faculty::new("20 - INGENIERÍA DE SISTEMAS E INFORMÁTICA".to_string()).unwrap(),
        program: Program::new("2 - E.P. de Ingeniería de Software".to_string()).unwrap(),
        specialty: Specialty::new("0 - Estudios Generales".to_string()).unwrap(),
    };

    debug_course(&course);
    
    println!("=========================================");
    println!("=========================================");

    debug_user(&user);
}


fn debug_course(course: &Course) {
    println!("Code: {:?}", course.code.value());
    println!("Name: {:?}", course.name.value());
    println!("Section: {:?}", course.section.value());
    println!("Curriculum: {:?}", course.curriculum);
    println!("Capacity: {:?}", course.capacity);
    println!("Credits: {:?}", course.credits.value());
    println!("Prerequisites: {:?}", course.prerequisites);
    println!("Teacher ID: {:?}", course.teacher_id);
    println!("Facility ID: {:?}", course.facility_id);
    println!("Cycle: {:?}", course.cycle);
    println!("Enrolled: {:?}", course.enrolled);
    println!("Schedule: {:?}", course.schedule);
}

fn debug_user(user: &User) {
    println!("Code: {:?}", user.code.value());
    println!("Faculty: {:?}", user.faculty);
    println!("Program: {:?}", user.program);
    println!("Specialty: {:?}", user.specialty);
}