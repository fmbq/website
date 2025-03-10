-- quizzer, quizmaster, coach, conference_director, regional_director, director, guest
CREATE TABLE person_type (
    id INTEGER NOT NULL PRIMARY KEY,
    name TEXT NOT NULL
);

INSERT INTO person_type VALUES(0,'QUIZZER');


CREATE TABLE person (
    id INTEGER NOT NULL PRIMARY KEY,
    firstname TEXT NOT NULL,
    middle TEXT,
    lastname TEXT NOT NULL,
    grade INTEGER NOT NULL,
    church_id INTEGER NOT NULL,
    sex INTEGER, -- 0 male, 1 female
    address1 TEXT,
    address2 TEXT,
    city TEXT,
    state TEXT,
    zip INTEGER,
    phone TEXT,
    email TEXT,
    birthdate TEXT,
    food_allergy TEXT -- json object with bools for FDA Food Allergies and an other string item
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
