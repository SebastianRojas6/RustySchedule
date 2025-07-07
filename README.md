<h1 align="center" style="font-weight: bold;">API de Matrículas 📚</h1>

<p align="center">
 <a href="#tech">Technologies</a> • 
 <a href="#started">Getting Started</a> • 
 <a href="#routes">API Endpoints</a> •
 <a href="#colab">Collaborators</a> •
 <a href="#contribute">Contribute</a>
</p>

<p align="center">
    <b>Sistema de gestión de matrículas estudiantiles desarrollado con tecnologías modernas</b>
</p>

<h2 id="technologies">💻 Technologies</h2>

- SeaORM
- Actixweb
- PostgreSQL

<h2 id="started">🚀 Getting Started</h2>

Asegúrat de tener las credenciales de supabase 

<h2 id="routes">📍 API Endpoints</h2>

### 📋 Métodos GET

#### 🔍 Obtener matrícula específica
- **Endpoint:** `/enrollments/student/{student_id}/course/{course_id}`
- **Descripción:** Devuelve la matrícula de un curso específico

```json
{
  "id": "4377b566-a66d-4904-81b0-d78c2f2c4126",
  "student_id": "1",
  "course_id": "COURSE002",
  "status": "Enrolled"
}
```

#### 📚 Obtener matrículas actuales
- **Endpoint:** `/enrollments/student/{student_id}/current`
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

#### ✅ Obtener cursos completados
- **Endpoint:** `/enrollments/student/{student_id}/completed`
- **Descripción:** Devuelve la lista de cursos completados

```json
[
  "COURSE002"
]
```

#### 🎯 Obtener créditos matriculados
- **Endpoint:** `/enrollments/student/{student_id}/credits`
- **Descripción:** Devuelve el número de créditos actualmente matriculados

```json
4
```

#### 📊 Obtener historial completo de matrículas
- **Endpoint:** `/enrollments/student/{student_id}/enrollments`
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

#### 👤 Obtener información del estudiante

- **Endpoint:** `GET /enrollments/student/{student_id}/info`  
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

### ✏️ Métodos POST

#### ✅ Completar curso
- **Endpoint:** `POST /enrollments/{id_enrollment}/complete`
- **Descripción:** Marca un curso como completado

#### 📝 Matricular estudiante
- **Endpoint:** `POST /enrollments/student/course/enroll`
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

#### ❌ Retirar de curso
- **Endpoint:** `POST /enrollments/{id_enrollment}/withdraw`
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

#### 📌 Cursos disponibles por semestre

- **Endpoint:** `GET /enrollments/student/available-courses?semester=2025-1`  
- **Descripción:** Lista todos los cursos disponibles para un semestre específico.

#### 📋 Ejemplo de respuesta

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

#### 🧪 Validar matrícula por curso

- **Endpoint:** `POST /enrollments/student/validate`  
- **Descripción:** Valida si un estudiante puede matricularse en un curso específico. Ideal para usar antes de ejecutar la matrícula.

**Ejemplo de JSON para matrícula:**

```json
{
  "student_id": "2",
  "course_id": "COURSE011",
  "section_id": "1"
}

```

### ✅ Posibles respuestas

| Código HTTP           | Mensaje                                     | Descripción                                                  |
|-----------------------|---------------------------------------------|--------------------------------------------------------------|
| `200 OK`              | `Valid enrollment`                          | El estudiante puede matricularse sin restricciones.          |
| `400 Bad Request`     | `Validation failed: Course not found`       | El curso especificado no existe.                             |
| `400 Bad Request`     | `Validation failed: Already enrolled`       | El estudiante ya está matriculado en el curso.               |
| `400 Bad Request`     | `Validation failed: Prerequisites not met`  | No se han cumplido los prerrequisitos del curso.             |
| `400 Bad Request`     | `Validation failed: Schedule conflict`      | Hay conflicto de horario con otro curso matriculado.         |
| `400 Bad Request`     | `Validation failed: Section full`           | La sección no tiene cupos disponibles.                       |
| `400 Bad Request`     | `Validation failed: Already passed course`  | El estudiante ya aprobó este curso previamente.              |
| `500 Internal Server Error` | `Internal server error`              | Error inesperado del sistema.                                |


### API para profesores

### 1. Obtener cursos asignados a un docente

**GET** `/teacher/courses/of-user/{teacher_id}`

**Descripción:**  
Obtiene todos los cursos activos que tiene asignado un docente específico.

**Parámetros:**

- `teacher_id` (string): Código del docente.

**Respuesta exitosa:**
```json
[
    {
        "id": "COURSE003",
        "code": "INE002",
        "name": "PROGRAMACIÓN Y COMPUTACIÓN",
        "section": 5,
        "curriculum": "Elective",
        "capacity": 40,
        "credits": 2,
        "hours_per_week": 3,
        "cycle": 1,
        "teacher_id": "085774",
        "facility_id": "",
        "enrolled": 1,
        "semester": "2025-1",
        "academic_year": 2025,
        "active": true
    },
    {
        "id": "COURSE015",
        "code": "202W0301",
        "name": "ALGORITMICA I",
        "section": 2,
        "curriculum": "Prerequisite",
        "capacity": 40,
        "credits": 4,
        "hours_per_week": 5,
        "cycle": 3,
        "teacher_id": "085774",
        "facility_id": "",
        "enrolled": 0,
        "semester": "2025-1",
        "academic_year": 2025,
        "active": true
    }
]
```

**Nota:** El `teacher_id` debe de ser el id del docente según la database.

---

### 2. Obtener horarios sugeridos para un docente

**GET** `/teacher/schedules/suggest/{teacher_id}`

**Descripción:**  
Devuelve una lista de horarios sugeridos para los cursos asignados al docente.

**Parámetros:**

- `teacher_id` (string): Código del docente.

**Respuesta exitosa:**
```json
[
    {
        "id": "suggested-1",
        "course_id": "COURSE003",
        "day": "Monday",
        "start_time": "08:00:00",
        "end_time": "11:00:00",
        "session_type": "Theory",
        "location_detail": null,
        "created_at": null,
        "facility_id": "211"
    },
    {
        "id": "suggested-2",
        "course_id": "COURSE015",
        "day": "Tuesday",
        "start_time": "13:00:00",
        "end_time": "18:00:00",
        "session_type": "Theory",
        "location_detail": null,
        "created_at": null,
        "facility_id": "211"
    }
]
```

**Nota:** El `teacher_id` debe de ser el id del docente según la database.

---

### 3. Registrar un horario

**POST** `/teacher/schedules`

**Descripción:**  
Registra un nuevo horario para un curso específico del docente.

**Body (JSON):**
```json
{
    "id": "suggested-2",
    "course_id": "COURSE015",
    "day": "Tuesday",
    "start_time": "13:00:00",
    "end_time": "16:00:00",
    "session_type": "Theory",
    "location_detail": null,
    "created_at": null,
    "facility_id": "211"
}
```

**Nota:** En caso tengas un vector de Schedules, entonces será enviar cada schedule por separado en esta ruta.

---

### 4. Actualizar una sugerencia de horario

**PUT** `/teacher/schedules/{schedule_id}`

**Descripción:**  
Modifica una sugerencia existente de horario para ajustarse a la disponibilidad del docente o instalaciones.

**Parámetros:**

- `schedule_id` (string): ID del horario sugerido.

**Body (JSON):**
```json
{
    "id": "suggested-2",
    "course_id": "COURSE015",
    "day": "Tuesday",
    "start_time": "13:00:00",
    "end_time": "16:00:00",
    "session_type": "Theory",
    "location_detail": null,
    "created_at": null,
    "facility_id": "211"
}
```

**Nota:** el `schedule_id` que se debe de pasar es el "id" del schedule según la db

---

### 5. Conocer si un usuario tiene horario asignado

**PUT** `/teacher/schedules/verify/{user_id}`

**Descripción:**  
Simplemente devuelve un bool donde `false` indica que no tiene ningún horario designado y `true` lo contrario.

**Parámetros:**

- `user_id` (string): ID del usuario a verificar

**Respuesta exitosa:**
```json
true
```

**Nota:** el `user_id` que se debe de pasar es el "id" del schedule según la db

---