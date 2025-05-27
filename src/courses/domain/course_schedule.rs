#[derive(Debug, Clone)]
pub enum Day {
    Lunes,
    Martes,
    Miércoles,
    Jueves,
    Viernes,
    Sábado,
    Domingo,
}

#[derive(Debug, Clone)]
pub enum SessionType {
    Teoria,
    Laboratorio,
}

#[derive(Debug, Clone)]
pub struct Schedule {
    pub day: Day,
    pub start_time: String,
    pub end_time: String,
    pub room: String,
    pub session_type: SessionType,
}
