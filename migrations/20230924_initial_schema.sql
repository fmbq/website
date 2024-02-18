CREATE TABLE user (
    id TEXT NOT NULL PRIMARY KEY,
    email TEXT NOT NULL UNIQUE,
    password_hash TEXT,
    require_password_reset INTEGER
);

CREATE TABLE reset_password_token (
    user_id TEXT NOT NULL PRIMARY KEY,
    token TEXT NOT NULL UNIQUE,
    created_time TEXT NOT NULL
);
