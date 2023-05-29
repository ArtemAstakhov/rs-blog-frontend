-- migrate:up
CREATE TABLE IF NOT EXISTS public.goal_folders
(
    id integer NOT NULL DEFAULT nextval('goal_folders_id_seq'::regclass),
    name text COLLATE pg_catalog."default" NOT NULL,
    CONSTRAINT goal_folders_pkey PRIMARY KEY (id)
)
WITH (
    OIDS = FALSE
)
TABLESPACE pg_default;

-- migrate:down
DROP TABLE public.goal_folders;