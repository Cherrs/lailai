-- Table: public.cache

-- DROP TABLE IF EXISTS public.cache;

CREATE TABLE IF NOT EXISTS public.cache
(
    code character varying(50) COLLATE pg_catalog."default" NOT NULL,
    datetime bigint NOT NULL,
    CONSTRAINT cache_pkey PRIMARY KEY (code)
)

TABLESPACE pg_default;

ALTER TABLE IF EXISTS public.cache
    OWNER to postgres;