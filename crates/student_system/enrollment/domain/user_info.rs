use serde::Serialize;

#[derive(Serialize)]
pub struct UserInfoResponse {
    pub code: String,
    pub email: Option<String>,
    pub specialty: String,
    pub full_name: String,
    pub semester: String,
    pub cursos_matriculados: usize,
    pub creditos_totales: i32,
}
