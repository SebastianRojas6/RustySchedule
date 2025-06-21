-- Esto es real data del sum xd

-- Profesores

INSERT INTO public.users (
  id, code, faculty, program, specialty, role, student_status,
  contract_type, hire_date, max_hours_per_week
)
VALUES 
('0A0182', 'DOC001', 'Ingeniería de Sistemas', 'Ingeniería de Software', 'Estudios Generales',
 'teacher', NULL, 'principal', '2015-03-01', 16),

('07240690', 'DOC002', 'Ingeniería de Sistemas', 'Ingeniería de Software', 'Estudios Generales',
 'teacher', NULL, 'contratado', '2021-04-01', 10),

('085774', 'DOC003', 'Ingeniería de Sistemas', 'Ingeniería de Software', 'Estudios Generales',
 'teacher', NULL, 'contratado', '2021-01-10', 8),

('10645375', 'DOC004', 'Ingeniería de Sistemas', 'Ingeniería de Software', 'Estudios Generales',
 'teacher', NULL, 'contratado', '2019-08-15', 16),

('15584946', 'DOC005', 'Ingeniería de Sistemas', 'Ingeniería de Software', 'Estudios Generales',
 'teacher', NULL, 'contratado', '2020-05-20', 12),

('42229911', 'DOC006', 'Ingeniería de Sistemas', 'Ingeniería de Software', 'Estudios Generales',
 'teacher', NULL, 'contratado', '2022-01-15', 10),

('41388541', 'DOC007', 'Ingeniería de Sistemas', 'Ingeniería de Software', 'Estudios Generales',
 'teacher', NULL, 'contratado', '2018-03-01', 16),

('21571331', 'DOC008', 'Ingeniería de Sistemas', 'Ingeniería de Software', 'Estudios Generales',
 'teacher', NULL, 'contratado', '2023-02-01', 6),

('40667934', 'DOC009', 'Ingeniería de Sistemas', 'Ingeniería de Software', 'Estudios Generales',
 'teacher', NULL, 'contratado', '2022-09-01', 10);


-- Aulas

INSERT INTO public.facilities (id, name, capacity)
VALUES 
('211', 'Aula 211', 40),
('109', 'Aula 109', 40),
('NP-108', 'Aula NP-108', 40),
('NP-109', 'Aula NP-109', 40),
('101', 'Aula 101', 40),
('212', 'Aula 212', 40),
('NP-102', 'Aula NP-102', 40),
('NP-103', 'Aula NP-103', 40);

-- Cursos

INSERT INTO public.courses (
  id, code, name, section, curriculum, capacity, credits, hours_per_week,
  cycle, teacher_id, facility_id, semester, academic_year
) VALUES
-- INE002 - PROGRAMACIÓN Y COMPUTACIÓN
('COURSE001', 'INE002', 'PROGRAMACIÓN Y COMPUTACIÓN', 2, 'obligatory', 40, 2, 3, 1, '0A0182', '211', '2025-1', 2025),
('COURSE002', 'INE002', 'PROGRAMACIÓN Y COMPUTACIÓN', 3, 'obligatory', 40, 2, 3, 1, '07240690', '211', '2025-1', 2025),
('COURSE003', 'INE002', 'PROGRAMACIÓN Y COMPUTACIÓN', 5, 'obligatory', 40, 2, 3, 1, '085774', '211', '2025-1', 2025),
('COURSE004', 'INE002', 'PROGRAMACIÓN Y COMPUTACIÓN', 6, 'obligatory', 40, 2, 3, 1, '10645375', '109', '2025-1', 2025),

-- INO101 - REDACCIÓN Y TÉCNICAS DE COMUNICACIÓN EFECTIVA I
('COURSE005', 'INO101', 'REDACCIÓN Y TÉCNICAS DE COMUNICACIÓN EFECTIVA I', 4, 'obligatory', 40, 3, 4, 1, '15584946', 'NP-108', '2025-1', 2025),
('COURSE006', 'INO101', 'REDACCIÓN Y TÉCNICAS DE COMUNICACIÓN EFECTIVA I', 5, 'obligatory', 40, 3, 4, 1, '42229911', 'NP-109', '2025-1', 2025),

-- INO102 - MÉTODOS DE ESTUDIO UNIVERSITARIO
('COURSE007', 'INO102', 'MÉTODOS DE ESTUDIO UNIVERSITARIO', 1, 'obligatory', 40, 2, 3, 1, '42229911', '101', '2025-1', 2025),

-- INO103 - DESARROLLO PERSONAL Y LIDERAZGO
('COURSE008', 'INO103', 'DESARROLLO PERSONAL Y LIDERAZGO', 6, 'obligatory', 40, 2, 3, 1, '41388541', '212', '2025-1', 2025),

-- INO106 - ÁLGEBRA Y GEOMETRÍA ANALÍTICA
('COURSE009', 'INO106', 'ÁLGEBRA Y GEOMETRÍA ANALÍTICA', 1, 'obligatory', 40, 4, 6, 1, '21571331', 'NP-102', '2025-1', 2025),
('COURSE010', 'INO106', 'ÁLGEBRA Y GEOMETRÍA ANALÍTICA', 2, 'obligatory', 40, 4, 6, 1, '40667934', 'NP-103', '2025-1', 2025);


-- 