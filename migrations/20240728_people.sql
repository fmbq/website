-- quizzer, quizmaster, coach, conference_director, regional_director, director, guest
CREATE TABLE person_type (
    id INTEGER NOT NULL PRIMARY KEY,
    name TEXT NOT NULL
);

CREATE TABLE person (
    id INTEGER NOT NULL PRIMARY KEY,
    firstname TEXT NOT NULL,
    middle TEXT,
    lastname TEXT NOT NULL,
    common_name TEXT NOT NULL, --name the user wants to go by
    sex CHAR(1) NOT NULL, -- 'M' male, 'F' female
    address1 TEXT,
    address2 TEXT,
    city TEXT,
    state CHAR(2),
    zip INTEGER,
    phone TEXT,
    email TEXT,
    birthdate TEXT,
    food_allergy JSON -- json object with bools for FDA Food Allergies and an 'other' string item
);


CREATE TABLE person_person_type (
    person_id INTEGER NOT NULL, --FK person
    person_type_id INTEGER NOT NULL, --FK person_type
    FOREIGN KEY(person_id) REFERENCES person(id),
    FOREIGN KEY(person_type_id) REFERENCES person_type(id)
);

CREATE TABLE conference (
    id INTEGER NOT NULL PRIMARY KEY,
    name TEXT NOT NULL
);

CREATE TABLE church (
    id INTEGER NOT NULL PRIMARY KEY,
    conference_id INTEGER NOT NULL, --FK conference
    title TEXT NOT NULL, -- official name of church, Compass Church
    name TEXT NOT NULL, -- name of church for tournaments, Janesville, needs to be unique
    address TEXT NOT NULL,
    city TEXT NOT NULL,
    state TEXT NOT NULL,
    zip INTEGER NOT NULL,
    url TEXT,
    FOREIGN KEY (conference_id) REFERENCES conference(id)
);

