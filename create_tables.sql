BEGIN;


CREATE TABLE IF NOT EXISTS public.chats
(
    chat_id integer NOT NULL DEFAULT nextval('chats_chat_id_seq'::regclass),
    app_user integer NOT NULL,
    created_on timestamp with time zone NOT NULL DEFAULT now(),
    chat_name character varying(255) COLLATE pg_catalog."default" NOT NULL DEFAULT ('New chat '::text || nextval('chats_chat_id_seq'::regclass)),
    CONSTRAINT chats_pkey PRIMARY KEY (chat_id)
);

CREATE TABLE IF NOT EXISTS public.messages
(
    id integer NOT NULL DEFAULT nextval('messages_id_seq'::regclass),
    created_on timestamp with time zone NOT NULL DEFAULT now(),
    role character varying(255) COLLATE pg_catalog."default",
    content text COLLATE pg_catalog."default",
    chat_id_relation integer,
    CONSTRAINT messages_pkey PRIMARY KEY (id)
);

ALTER TABLE IF EXISTS public.messages
    ADD CONSTRAINT messages_chat_id_relation_fkey FOREIGN KEY (chat_id_relation)
    REFERENCES public.chats (chat_id) MATCH SIMPLE
    ON UPDATE NO ACTION
    ON DELETE CASCADE;

END;
