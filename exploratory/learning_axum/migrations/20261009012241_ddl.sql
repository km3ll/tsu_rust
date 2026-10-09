-- Add migration script here
CREATE TABLE IF NOT EXISTS zettels (
    id serial NOT NULL PRIMARY KEY,
    content text NOT NULL
);