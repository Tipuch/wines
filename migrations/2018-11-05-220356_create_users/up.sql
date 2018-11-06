CREATE TABLE users
(
    id SERIAL PRIMARY KEY,
    email varchar NOT NULL UNIQUE CHECK (email <> ''),
    admin boolean NOT NULL DEFAULT false,
    salt bytea NOT NULL,
    password bytea NOT NULL
);
CREATE INDEX users_email ON users (email);