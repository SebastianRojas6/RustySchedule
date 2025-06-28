ALTER TABLE public.users
ADD COLUMN full_name VARCHAR,
ADD COLUMN password VARCHAR DEFAULT 'changeme' NOT NULL;
