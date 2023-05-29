-- migrate:up
CREATE TABLE IF NOT EXISTS public.goals
(
    id integer NOT NULL DEFAULT nextval('goals_id_seq'::regclass),
    title text COLLATE pg_catalog."default" NOT NULL,
    completed_at timestamp without time zone,
    created_at timestamp without time zone NOT NULL DEFAULT now(),
    updated_at timestamp without time zone NOT NULL DEFAULT now(),
    folder_id integer,
    CONSTRAINT goals_pkey PRIMARY KEY (id),
    CONSTRAINT goals_goal_folder_fkey FOREIGN KEY (folder_id)
        REFERENCES public.goal_folders (id) MATCH SIMPLE
        ON UPDATE NO ACTION
        ON DELETE SET NULL
        NOT VALID
)
WITH (
    OIDS = FALSE
)
TABLESPACE pg_default;

-- migrate:down
DROP TABLE public.goals;