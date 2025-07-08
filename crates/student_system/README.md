<h2 id="routes">📍 API Endpoints - Students</h2>

# API para estudiantes

## 📋 Métodos GET

### 🧾 Obtener secciones disponibles por código de curso

- **Endpoint:** `GET /student/available-sections/{course_code}`  
- **Descripción:** Devuelve la información del curso y sus secciones disponibles, incluyendo horarios, docentes y vacantes.

**Ejemplo de respuesta:**

```json
{
  "nombre_curso": "INTRODUCCION AL DESARROLLO DE SOFTWARE",
  "codigo": "202W0305",
  "num_ciclo": 3,
  "creditos": 3,
  "num_secc": 3,
  "secciones": [
    {
      "available_spots": 40,
      "teacher_id": "0A7624",
      "day": "Wednesday",
      "session_type": "Theory",
      "start_time": "13:00",
      "end_time": "14:00"
    },
    {
      "available_spots": 40,
      "teacher_id": "09571E",
      "day": "Thursday",
      "session_type": "Lab",
      "start_time": "09:00",
      "end_time": "10:00"
    },
    {
      "available_spots": 40,
      "teacher_id": "0A0314",
      "day": "Thursday",
      "session_type": "Theory",
      "start_time": "15:00",
      "end_time": "17:00"
    }
  ]
}

### 🧭 Obtener sugerencia de horario

- **Endpoint:** `GET /student/{student_id}/schedule-suggestion`  
- **Descripción:** Devuelve una lista sugerida de secciones para el estudiante indicado, optimizadas para su carga académica y disponibilidad.

```json
{
  "selected_sections": [
    "INO102",
    "INO103",
    "INO106",
    "INO106",
    "INO104",
    "INE002",
    "INO101",
    "INO101"
  ]
}
```

### 🔍 Obtener matrícula específica
- **Endpoint:** `/student/{student_id}/course/{course_id}`
- **Descripción:** Devuelve la matrícula de un curso específico

```json
{
  "id": "4377b566-a66d-4904-81b0-d78c2f2c4126",
  "student_id": "1",
  "course_id": "COURSE002",
  "status": "Enrolled"
}
```

### 📌 Cursos disponibles por semestre

- **Endpoint:** `GET /student/available-courses?semester=2025-1`  
- **Descripción:** Lista todos los cursos disponibles para un semestre específico.

### 📋 Ejemplo de respuesta

```json
[
  {
    "id": "COURSE007",
    "code": "INO102",
    "name": "MÉTODOS DE ESTUDIO UNIVERSITARIO",
    "credits": 2,
    "cycle": 1,
    "teacher_id": "42229911",
    "section": 1,
    "curriculum": "obligatory"
  },
  {
    "id": "COURSE008",
    "code": "INO103",
    "name": "DESARROLLO PERSONAL Y LIDERAZGO",
    "credits": 2,
    "cycle": 1,
    "teacher_id": "41388541",
    "section": 6,
    "curriculum": "obligatory"
  }
  // ... etc
]
```

### 📚 Obtener matrículas actuales
- **Endpoint:** `/student/{student_id}/current`
- **Descripción:** Devuelve todos los cursos en los que está actualmente matriculado

```json
[
  {
    "id": "4377b566-a66d-4904-81b0-d78c2f2c4126",
    "student_id": "1",
    "course_id": "COURSE002",
    "status": "Enrolled"
  },
  {
    "id": "121de0dd-f707-43bf-8a67-252958e08d1a",
    "student_id": "1",
    "course_id": "COURSE003",
    "status": "Enrolled"
  }
]
```

### ✅ Obtener cursos completados
- **Endpoint:** `/student/{student_id}/completed`
- **Descripción:** Devuelve la lista de cursos completados

```json
[
  "COURSE002"
]
```

### 🎯 Obtener créditos matriculados
- **Endpoint:** `/student/{student_id}/credits`
- **Descripción:** Devuelve el número de créditos actualmente matriculados

```json
4
```
### 📊 Obtener historial completo de matrículas
- **Endpoint:** `/student/{student_id}/enrollments`
- **Descripción:** Devuelve el historial completo de matrículas con detalles del curso

```json
[
  {
    "course_id": "COURSE002",
    "name": "PROGRAMACIÓN Y COMPUTACIÓN",
    "status": "Enrolled",
    "credits": 2
  },
  {
    "course_id": "COURSE003",
    "name": "PROGRAMACIÓN Y COMPUTACIÓN",
    "status": "Enrolled",
    "credits": 2
  },
  {
    "course_id": "COURSE004",
    "name": "PROGRAMACIÓN Y COMPUTACIÓN",
    "status": "Withdrawn",
    "credits": 2
  },
  {
    "course_id": "COURSE001",
    "name": "PROGRAMACIÓN Y COMPUTACIÓN",
    "status": "Withdrawn",
    "credits": 2
  }
]
```

### 👤 Obtener información del estudiante

- **Endpoint:** `GET /student/{student_id}/info`  
- **Descripción:** Retorna información general del estudiante, incluyendo su código, correo, especialidad, cantidad de cursos matriculados y créditos totales en el semestre actual.

```json
{
  "code": "22200002",
  "email": "a02@unmsm.edu.pe",
  "specialty": "Ingeniería de software",
  "full_name": "",
  "semester": "2025-1",
  "cursos_matriculados": 5,
  "creditos_totales": 12
}
```

## ✏️ Métodos POST

### ✅ Completar curso
- **Endpoint:** `POST /student/{id_enrollment}/complete`
- **Descripción:** Marca un curso como completado

### 📝 Matricular estudiante
- **Endpoint:** `POST /student/course/enroll`
- **Descripción:** Crea una nueva matrícula para un estudiante

**Ejemplo de JSON para matrícula:**
```json
{
  "student_id": "4",
  "course_id": "COURSE001",
  "student_curriculum": "2023",
  "course_curriculum": "2023",
  "student_status": "regular",
  "student_credits_enrolled": 12,
  "course_credits": 3,
  "completed_courses": [],
  "course_prerequisites": [],
  "course_cycle": 1,
  "semester": "Odd",
  "section_capacity_available": true,
  "already_enrolled": false,
  "times_repeated": 0,
  "schedule_conflict": false,
  "course_already_passed": false
}
```

### ❌ Retirar de curso
- **Endpoint:** `POST /student/{id_enrollment}/withdraw`
- **Descripción:** Retira a un estudiante de un curso específico

**Estados de Matrícula**

- `Enrolled`: Estudiante matriculado activamente
- `Completed`: Curso completado exitosamente
- `Withdrawn`: Estudiante retirado del curso

**Códigos de Respuesta**

- `200 OK`: Solicitud exitosa
- `201 Created`: Recurso creado exitosamente
- `400 Bad Request`: Datos de entrada inválidos
- `404 Not Found`: Recurso no encontrado
- `500 Internal Server Error`: Error interno del servidor

### 🧪 Validar matrícula por curso

- **Endpoint:** `POST /student/validate`  
- **Descripción:** Valida si un estudiante puede matricularse en un curso específico. Ideal para usar antes de ejecutar la matrícula.

**Ejemplo de JSON para matrícula:**

```json
{
  "student_id": "2",
  "course_id": "COURSE011",
  "section_id": "1"
}

```

#### ✅ Posibles respuestas

| Código HTTP           | Mensaje                                     | Descripción                                                  |
|-----------------------|---------------------------------------------|--------------------------------------------------------------|
| `200 OK`              | `Valid enrollment`                          | El estudiante puede matricularse sin restricciones.          |
| `400 Bad Request`     | `Validation failed: Course not found`       | El curso especificado no existe.                             |
| `400 Bad Request`     | `Validation failed: Already enrolled`       | El estudiante ya está matriculado en el curso.               |
| `400 Bad Request`     | `Validation failed: Prerequisites not met`  | No se han cumplido los prerrequisitos del curso.             |
| `400 Bad Request`     | `Validation failed: Schedule conflict`      | Hay conflicto de horario con otro curso matriculado.         |
| `400 Bad Request`     | `Validation failed: Section full`           | La sección no tiene cupos disponibles.                       |
| `400 Bad Request`     | `Validation failed: Already passed course`  | El estudiante ya aprobó este curso previamente.              |
| `500 Internal Server Error` | `Internal server error`              | Error inesperado del sistema.                               

### 📅 Registrar disponibilidad horaria del estudiante

- **Endpoint:** `POST /student/{student_id}/availability`  
- **Descripción:** Registra la disponibilidad semanal del estudiante, indicando los días y horas en que está disponible para asistir a clases.

**Ejemplo de JSON de entrada:**

```json
{
  "availabilities": [
    {
      "day": "monday",
      "start_time": "08:00",
      "end_time": "22:00"
    },
    {
      "day": "tuesday",
      "start_time": "08:00",
      "end_time": "22:00"
    },
    {
      "day": "wednesday",
      "start_time": "08:00",
      "end_time": "22:00"
    },
  ]
}
