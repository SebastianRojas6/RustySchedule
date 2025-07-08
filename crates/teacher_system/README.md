<h2 id="routes">📍 API Endpoints - Teachers</h2>

# API para profesores

## 📋 Métodos GET

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

### 3. Conocer si un usuario tiene horario asignado

**GET** `/teacher/schedules/verify/{user_id}`

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

### 4. Obtener todas los horarios disponibles

**GET** `/teacher/facilities/all/available`

**Descripción:**  
Devuelve todos los salones que están disponibles por cada día de Lunes a Sábado y que horas tiene disponible cada día.

**Respuesta exitosa:**
```json
[
    {
        "facility": {
            "id": "211",
            "name": "Aula 211",
            "capacity": 40,
            "facility_type": "classroom",
            "created_at": "2025-06-21 17:40:39.813646"
        },
        "available_slots": {
            "Monday": [
                [
                    "11:00:00",
                    "12:00:00"
                ],
                [
                    "13:00:00",
                    "14:00:00"
                ],
                [
                    "16:00:00",
                    "22:00:00"
                ]
            ],
        .
        .
        .
        }
    }
]
```

---

## 📋 Métodos POST


### 5. Registrar un horario

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

## 📋 Métodos PUT


### 6. Actualizar una sugerencia de horario

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