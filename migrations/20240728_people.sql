// quizzer, quizmaster, coach, conference_director, regional_director, director, guest
CREATE TABLE person_type (
    id INTEGER NOT NULL PRIMARY KEY,
    name TEXT NOT NULL
);

CREATE TABLE person (
    id INTEGER NOT NULL PRIMARY KEY,
    firstname TEXT NOT NULL,
    lastname TEXT NOT NULL,
    middle TEXT,
    grade INTEGER NOT NULL,
    church_id INTEGER NOT NULL
);

CREATE TABLE person_person_type (
    person_id INTEGER NOT NULL,
    person_type_id INTEGER NOT NULL

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
