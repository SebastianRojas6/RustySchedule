use super::{
    user_faculty::Faculty,
    user_program::Program,
    user_specialty::Specialty,
    user_code::UserCode,
};

#[derive(Debug, Clone)]
pub struct User {
    pub code: UserCode,
    pub faculty: Faculty,
    pub program: Program,
    pub specialty: Specialty,
}
