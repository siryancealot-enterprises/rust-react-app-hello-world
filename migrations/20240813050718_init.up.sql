-- Initial DB script

CREATE TABLE IF NOT EXISTS player
(
    id uuid NOT NULL DEFAULT gen_random_uuid(), -- Postgres defaults this to v4
    name varchar(32) NOT NULL,
    number integer NOT NULL,
    username varchar(32) UNIQUE NOT NULL,
    email varchar(32) NOT NULL,
    CONSTRAINT players_pkey PRIMARY KEY ("id")
)

