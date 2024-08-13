-- Initial DB script

CREATE TABLE IF NOT EXISTS player
(
    number integer NOT NULL,
    name varchar(32) NOT NULL,
    username varchar(32) NOT NULL,
    email varchar(32) NOT NULL,
    CONSTRAINT players_pkey PRIMARY KEY ("number")
)

