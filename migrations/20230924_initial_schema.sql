CREATE TABLE user (
    id TEXT NOT NULL PRIMARY KEY,
    email TEXT NOT NULL UNIQUE,
    password_hash TEXT,
    require_password_reset INTEGER
);
