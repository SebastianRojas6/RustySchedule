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
