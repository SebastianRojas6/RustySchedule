CREATE TABLE public.availabilities (
  id character varying NOT NULL,
  student_id character varying NOT NULL,
  day day_type NOT NULL, -- ← aquí corregido
  start_time time without time zone NOT NULL,
  end_time time without time zone NOT NULL,
  created_at timestamp without time zone DEFAULT CURRENT_TIMESTAMP,
  updated_at timestamp without time zone DEFAULT CURRENT_TIMESTAMP,
  CONSTRAINT availabilities_pkey PRIMARY KEY (id),
  CONSTRAINT availabilities_student_id_fkey FOREIGN KEY (student_id) REFERENCES public.users(id),
  CONSTRAINT availabilities_day_unique UNIQUE (student_id, day)
);
