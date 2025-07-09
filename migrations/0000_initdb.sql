create table public.course_prerequisites (
  course_id character varying(20) not null,
  prerequisite_course_id character varying(20) not null,
  requirement_type character varying(20) null default 'mandatory'::character varying,
  credits_required integer null default 0,
  created_at timestamp without time zone null default CURRENT_TIMESTAMP,
  constraint course_prerequisites_pkey primary key (course_id, prerequisite_course_id),
  constraint course_prerequisites_course_id_fkey foreign KEY (course_id) references courses (id) on delete CASCADE,
  constraint course_prerequisites_prerequisite_course_id_fkey foreign KEY (prerequisite_course_id) references courses (id) on delete CASCADE,
  constraint course_prerequisites_no_self_reference check (
    (
      (course_id)::text <> (prerequisite_course_id)::text
    )
  )
) TABLESPACE pg_default;


create table public.course_schedules (
  id character varying(20) not null,
  facilitie_id character varying(20) not null,
  day public.day_type not null,
  start_time time without time zone not null,
  end_time time without time zone not null,
  session_type public.session_type not null default 'theory'::session_type,
  location_detail character varying(100) null,
  created_at timestamp without time zone null default CURRENT_TIMESTAMP,
  constraint course_schedules_pkey primary key (id),
  constraint course_schedules_facilitie_id_fkey foreign KEY (facilitie_id) references facilities (id),
  constraint course_schedules_duration_check check (
    (
      (
        (
          EXTRACT(
            epoch
            from
              (end_time - start_time)
          ) / (3600)::numeric
        ) >= (1)::numeric
      )
      and (
        (
          EXTRACT(
            epoch
            from
              (end_time - start_time)
          ) / (3600)::numeric
        ) <= (4)::numeric
      )
    )
  ),
  constraint course_schedules_time_check check ((start_time < end_time))
) TABLESPACE pg_default;

create index IF not exists idx_course_schedules_course on public.course_schedules using btree (facilitie_id) TABLESPACE pg_default;

create index IF not exists idx_course_schedules_day on public.course_schedules using btree (day) TABLESPACE pg_default;

create index IF not exists idx_course_schedules_time_range on public.course_schedules using btree (day, start_time, end_time) TABLESPACE pg_default;

create table public.courses (
  id character varying(20) not null,
  code character varying(20) not null,
  name character varying(100) not null,
  section integer not null,
  curriculum public.curriculum_type not null,
  capacity integer not null,
  credits integer not null,
  hours_per_week integer not null default 2,
  cycle integer not null,
  teacher_id character varying(20) not null,
  schedule_id character varying(20) not null,
  enrolled integer not null default 0,
  max_capacity integer GENERATED ALWAYS as (capacity) STORED null,
  available_spots integer GENERATED ALWAYS as ((capacity - enrolled)) STORED null,
  semester character varying(10) not null default '2024-1'::character varying,
  academic_year integer not null default 2024,
  active boolean null default true,
  created_at timestamp without time zone null default CURRENT_TIMESTAMP,
  updated_at timestamp without time zone null default CURRENT_TIMESTAMP,
  constraint courses_pkey primary key (id),
  constraint courses_code_section_semester_unique unique (code, section, semester, academic_year),
  constraint courses_schedule_id_fkey foreign KEY (schedule_id) references course_schedules (id),
  constraint courses_teacher_id_fkey foreign KEY (teacher_id) references users (id) on delete RESTRICT,
  constraint courses_cycle_check check (
    (
      (cycle >= 1)
      and (cycle <= 12)
    )
  ),
  constraint courses_enrolled_check check (
    (
      (enrolled >= 0)
      and (enrolled <= capacity)
    )
  ),
  constraint courses_hours_check check (
    (
      (hours_per_week >= 1)
      and (hours_per_week <= 12)
    )
  ),
  constraint chk_courses_credits check (
    (
      (credits >= 1)
      and (credits <= 4)
    )
  ),
  constraint courses_year_check check (
    (
      (academic_year >= 2020)
      and (academic_year <= 2030)
    )
  ),
  constraint chk_courses_cycle check (
    (
      (cycle >= 1)
      and (cycle <= 10)
    )
  ),
  constraint courses_capacity_check check (
    (
      (capacity > 0)
      and (capacity <= 200)
    )
  ),
  constraint courses_credits_check check (
    (
      (credits >= 1)
      and (credits <= 6)
    )
  )
) TABLESPACE pg_default;

create index IF not exists idx_courses_teacher on public.courses using btree (teacher_id) TABLESPACE pg_default;

create index IF not exists idx_courses_facility on public.courses using btree (schedule_id) TABLESPACE pg_default;

create index IF not exists idx_courses_code_section on public.courses using btree (code, section) TABLESPACE pg_default;

create index IF not exists idx_courses_semester_year on public.courses using btree (semester, academic_year) TABLESPACE pg_default;

create index IF not exists idx_courses_cycle on public.courses using btree (cycle) TABLESPACE pg_default;

create index IF not exists idx_courses_available_spots on public.courses using btree (available_spots) TABLESPACE pg_default
where
  (available_spots > 0);

create trigger trigger_update_courses_timestamp BEFORE
update on courses for EACH row
execute FUNCTION update_updated_at_column ();


create table public.enrollments (
  id character varying(20) not null,
  student_id character varying(20) not null,
  course_id character varying(20) not null,
  status public.enrollment_status not null default 'enrolled'::enrollment_status,
  constraint enrollments_pkey primary key (id),
  constraint enrollments_student_course_unique unique (student_id, course_id),
  constraint enrollments_course_id_fkey foreign KEY (course_id) references courses (id) on delete RESTRICT,
  constraint enrollments_student_id_fkey foreign KEY (student_id) references users (id) on delete RESTRICT
) TABLESPACE pg_default;

create index IF not exists idx_enrollments_student on public.enrollments using btree (student_id) TABLESPACE pg_default;

create index IF not exists idx_enrollments_course on public.enrollments using btree (course_id) TABLESPACE pg_default;

create index IF not exists idx_enrollments_status on public.enrollments using btree (status) TABLESPACE pg_default;

create trigger trigger_update_enrolled_on_insert
after INSERT on enrollments for EACH row
execute FUNCTION update_course_enrolled ();

create trigger trigger_update_enrolled_on_update
after
update on enrollments for EACH row
execute FUNCTION update_course_enrolled ();

create trigger trigger_update_enrolled_on_delete
after DELETE on enrollments for EACH row
execute FUNCTION update_course_enrolled ();

create trigger trigger_update_enrollments_timestamp BEFORE
update on enrollments for EACH row
execute FUNCTION update_updated_at_column ();

create table public.facilities (
  id character varying(20) not null,
  name character varying(50) not null,
  capacity integer null default 0,
  facility_type character varying(30) null default 'classroom'::character varying,
  created_at timestamp without time zone null default CURRENT_TIMESTAMP,
  constraint facilities_pkey primary key (id),
  constraint facilities_capacity_check check ((capacity >= 0))
) TABLESPACE pg_default;

create table public.users (
  id character varying(20) not null,
  code character varying(20) not null,
  email character varying(255) null,
  phone character varying(20) null,
  faculty character varying(100) not null,
  program character varying(50) not null,
  specialty character varying(50) not null,
  student_status public.student_status null default 'regular'::student_status,
  admission_date date null,
  contract_type public.teacher_contract null,
  max_hours_per_week integer null,
  hire_date date null,
  active boolean null default true,
  constraint users_pkey primary key (id),
  constraint users_code_key unique (code),
  constraint users_email_key unique (email),
  constraint users_max_hours_check check (
    (
      (max_hours_per_week is null)
      or (
        (max_hours_per_week > 0)
        and (max_hours_per_week <= 20)
      )
    )
  )
) TABLESPACE pg_default;

create index IF not exists idx_users_code on public.users using btree (code) TABLESPACE pg_default;

create index IF not exists idx_users_faculty_program on public.users using btree (faculty, program) TABLESPACE pg_default;

create trigger trigger_update_users_timestamp BEFORE
update on users for EACH row
execute FUNCTION update_updated_at_column ();