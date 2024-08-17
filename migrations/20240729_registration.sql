CREATE TABLE tournament (
    id INTEGER NOT NULL PRIMARY KEY,
    name TEXT NOT NULL,
    date TEXT NOT NULL

);



-- which conferences are invited to which tournament
CREATE TABLE tournament_conference (
    tournament_id INTEGER NOT NULL,
    conference_id INTEGER NOT NULL

);

