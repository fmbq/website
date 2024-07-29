CREATE TABLE tournament (
    id INTEGER NOT NULL PRIMARY KEY,
    name TEXT NOT NULL,
    date TEXT NOT NULL,

);

CREATE TABLE person (
    id TEXT NOT NULL PRIMARY KEY,
    firstname TEXT NOT NULL,
    lastname TEXT NOT NULL,
    middle TEXT,
    grade INTEGER NOT NULL,
    church_id INTEGER NOT NULL
);

// which conferences are invited to which tournament
CREATE TABLE tournament_conference (
    tournament_id INTEGER NOT NULL,
    conference_id INTEGER NOT NULL

);

CREATE TABLE church (
    id INTEGER NOT NULL PRIMARY KEY,
    name TEXT NOT NULL,
    address TEXT NOT NULL,
    city TEXT NOT NULL,
    state TEXT NOT NULL,
    zip INTEGER NOT NULL,
    primary_contact INTEGER NOT NULL
);

CREATE TABLE conference (
    id INTEGER NOT NULL PRIMARY KEY,
    name TEXT NOT NULL,
    director INTEGER
);
