CREATE TABLE season (
    id INTEGER NOT NULL PRIMARY KEY,
    year INTEGER NOT NULL,
    material TEXT NOT NULL,
    translation TEXT NOT NULL
);

CREATE TABLE season_role (
    id INTEGER NOT NULL PRIMARY KEY,
    name TEXT NOT NULL
);

-- divisions only apply to single tournament
-- a quizzer might quiz up or might quiz in different divisions for team/individuals
CREATE TABLE division (
    id INTEGER NOT NULL PRIMARY KEY,
    name TEXT NOT NULL,
    grouping INTEGER NOT NULL -- this is so that division can be grouped as equal, e.g. YTV..YTVA..YTVB
);

-- churches are registered for a given a season
CREATE TABLE season_church (
    id INTEGER NOT NULL PRIMARY KEY,
    season_id INTEGER NOT NULL, --FK season
    primary_contact_id INTEGER NOT NULL, --FK person
    church_id INTEGER NOT NULL, --FK church
    FOREIGN KEY (season_id) REFERENCES season(id),
    FOREIGN KEY (primary_contact_id) REFERENCES person(id),
    FOREIGN KEY (church_id) REFERENCES church(id)
);

-- for a given a season, each person is associated with a church and a role
CREATE TABLE season_person (
    id INTEGER NOT NULL PRIMARY KEY,
    season_id INTEGER NOT NULL, --FK season
    person_id INTEGER NOT NULL, --FK person
    church_id INTEGER NOT NULL, --FK church
    division_id INTEGER NOT NULL, --FK division, quizzers have a base division for the season and can quiz below this division
    FOREIGN KEY (season_id) REFERENCES season(id),
    FOREIGN KEY (person_id) REFERENCES person(id),
    FOREIGN KEY (church_id) REFERENCES church(id),
    FOREIGN KEY (division_id) REFERENCES division(id)
);

-- quizmasters are allowed a preference for which divisions they want to quizmaster
-- database isn't going to enforce the person to be a quizmaster, that can be done in the software
CREATE TABLE season_quizmaster_division (
    quizmaster_id INTEGER NOT NULL, -- FK season_person, software enforces this to be a quizmaster
    division_id INTEGER NOT NULL, -- FK division, but the software will use the group, so just choose
    FOREIGN KEY (quizmaster_id) REFERENCES season_person(id)
    FOREIGN KEY (division_id) REFERENCES division(id)
);

-- base tournament table that contains the location and instructions for the quiz
CREATE TABLE tournament (
    id INTEGER NOT NULL PRIMARY KEY,
    name TEXT NOT NULL,
    date TEXT NOT NULL,
    church_id INTEGER NOT NULL, --FK church, this is the church where the tournament will be held, might not be a participating church
    details TEXT NOT NULL,
    costs TEXT NOT NULL,
    registration_open_date TEXT NOT NULL, -- date the registration should show up on the website
    registration_close_date TEXT NOT NULL, -- date new registrations are no longer allowed by the website
    FOREIGN KEY (church_id) REFERENCES church(id)
);

-- people that should be emailed tournament
CREATE TABLE tournament_registration_notifications (
    tournament_id INTEGER NOT NULL,
    person_id INTEGER NOT NULL, -- will use email address from this person
    FOREIGN KEY (tournament_id) REFERENCES tournament(id),
    FOREIGN KEY (person_id) REFERENCES season_person(id)
);


-- which conferences are invited to which tournament, but any church can go to any tournament
CREATE TABLE tournament_conference (
    id INTEGER NOT NULL PRIMARY KEY,
    tournament_id INTEGER NOT NULL, --FK tournament
    conference_id INTEGER NOT NULL,  --FK conference
    FOREIGN KEY (tournament_id) REFERENCES tournament(id),
    FOREIGN KEY (conference_id) REFERENCES conference(id)
);

-- these are the divisions allowed for the tournament
-- every tournament could have different divisions allowed
-- divisions apply to teams AND individuals
-- if using no-count rounds, use the division_id for '999'
CREATE TABLE tournament_division (
    id INTEGER NOT NULL PRIMARY KEY,
    tournament_id INTEGER NOT NULL, --FK tournament
    division_id INTEGER NOT NULL, --FK division
    FOREIGN KEY (tournament_id) REFERENCES tournament(id),
    FOREIGN KEY (division_id) REFERENCES division(id)
);



-----------------------------------------------------------------------
-----------------------------------------------------------------------
-- tournament registration tables
CREATE TABLE registration (
    id INTEGER NOT NULL PRIMARY KEY,
    code CHAR(20) NOT NULL UNIQUE, -- registration code that users can use to edit this registration, always a 20 character hash
    tournament_id INTEGER NOT NULL, -- FK tournament
    church_id INTEGER NOT NULL, -- FK season_church, this is the church that registered
    coach_id INTEGER NOT NULL, -- FK season_person, coach that registered
    seats INTEGER NOT NULL, -- number of quiz seats you are bringing
    meal_count INTEGER NOT NULL, -- people that are eating lunch, we can't presume that registered people are eating
    FOREIGN KEY (tournament_id) REFERENCES tournament(id),
    FOREIGN KEY (church_id) REFERENCES church(id),
    FOREIGN KEY (coach_id) REFERENCES season_person(id)
);


-- ALL quizzers must be registered in this table
CREATE TABLE registration_quizzer (
    id INTEGER NOT NULL PRIMARY KEY, -- use this for team composition
    registration_id INTEGER NOT NULL, -- FK registration, quizzer is associated with a specific registration
    quizzer_id INTEGER NOT NULL, -- FK season_person, quizzer is registerd to a church, but could be on a mixed team...
    tournament_division_id INTEGER NOT NULL, -- FK tournament division, for individuals, use 999 if not doing individuals
    no_show INTEGER NOT NULL DEFAULT 0, -- if the quizzer doesn't show up, use a flag to keep them out of stats and the tournament
    FOREIGN KEY (registration_id) REFERENCES registration(id),
    FOREIGN KEY (quizzer_id) REFERENCES season_person(id),
    FOREIGN KEY (tournament_division_id) REFERENCES tournament_division(id)
);

-- teams are composed of up to 6 individuals
-- team members might be from different churches, so take that into account
CREATE TABLE registration_team (
    id INTEGER NOT NULL PRIMARY KEY, -- use for results
    name TEXT NOT NULL, -- with multiple teams in the same division from the same church, provide a unique (fun) name
    tournament_division_id INTEGER NOT NULL, -- FK tournament division
    FOREIGN KEY (tournament_division_id) REFERENCES tournament_division(id)
);

-- quizzers on a team in a different table because we don't know the exact number
-- teams are composed of 2 to 6 individuals, the database doesn't need to enforce this
CREATE TABLE registration_team_quizzer (
    team_id INTEGER NOT NULL, -- FK registration_team id
    quizzer_id INTEGER NOT NULL, -- registration_quizzer id
    FOREIGN KEY (team_id) REFERENCES registration_team(id),
    FOREIGN KEY (quizzer_id) REFERENCES registration_quizzer(id)
);

-- need the quizmaster list, we don't need to track their preferred division, since that is a seasonal setup
CREATE TABLE registration_quizmaster (
    id INTEGER NOT NULL PRIMARY KEY, -- use this for results
    registration_id INTEGER NOT NULL, -- FK registration, quizmaster is associated with a specific registration
    quizmaster_id INTEGER NOT NULL, -- FK season_person
    FOREIGN KEY (registration_id) REFERENCES registration(id),
    FOREIGN KEY (quizmaster_id) REFERENCES season_person(id)
);


-----------------------------------------------------------------------
-----------------------------------------------------------------------
-- now the actual tournament setup
-- plan that each round could have a different quizmaster or multiple quizmasters
CREATE TABLE tournament_room (
    id INTEGER NOT NULL PRIMARY KEY,
    name TEXT NOT NULL -- name of the room, A, B, C, ...
);

CREATE TABLE tournament_round_team (
    id INTEGER NOT NULL PRIMARY KEY,
    round INTEGER NOT NULL, -- 1..n
    room_id INTEGER NOT NULL, -- FK tournament_room
    quizmaster_id INTEGER NOT NULL, -- FK registration_quizmaster, primary quizmaster
    consultant_id INTEGER, -- FK registration_quizmaster, quizmaster acting as a consultant
    scorekeeper TEXT, -- just keep the data since this could be anybody
    division_id INTEGER NOT NULL, -- FK tournament_division, could be a no-count or mixed divisions so we can't rely on the teams division, no count is always a division of '999'
    team1_id INTEGER NOT NULL, -- FK registration_team
    team1_coach INTEGER, --FK registration_person, this is a coach that is preset, the software will ensure it's a coach for the team
    team1_score INTEGER, -- store the overall score for fast retrieval
    team2_id INTEGER NOT NULL, -- FK registration_team
    team2_coach INTEGER, --FK registration_person, this is a coach that is preset, the software will ensure it's a coach for the team
    team2_score INTEGER, -- store the overall score for fast retrieval
    questions INTEGER NOT NULL DEFAULT 0, -- in case we want to publish real time scores, this is the current or final question number
    round_complete INTEGER NOT NULL DEFAULT 0, -- in case we want to publish real time scores
    ticker TEXT, -- contains the current state of the quiz as a single line, e.g. John called on to answer...John got question 10 correct...team 2 called a timeout... John prejumped
    results JSON, -- json containing each question, who prejumped and when, seat order, and all the details from the scoresheet
    FOREIGN KEY (room_id) REFERENCES tournament_room(id),
    FOREIGN KEY (quizmaster_id) REFERENCES registration_quizmaster(id),
    FOREIGN KEY (consultant_id) REFERENCES registration_quizmaster(id),
    FOREIGN KEY (division_id) REFERENCES tournament_division(id),
    FOREIGN KEY (team1_id) REFERENCES registration_team(id),
    FOREIGN KEY (team2_id) REFERENCES registration_team(id)
);

-- individuals is more complicated because now we allow up to 8 to quiz...
CREATE TABLE tournament_round_individuals (
    id INTEGER NOT NULL PRIMARY KEY,
    round INTEGER NOT NULL, -- 1 => prelim, 2=> semi-final 3=> final, basically, the highest number is the final round
    room_id INTEGER NOT NULL, -- FK tournament_room
    quizmaster_id INTEGER NOT NULL, -- FK registration_quizmaster
    consultant_id INTEGER, -- FK registration_quizmaster, quizmaster acting as a consultant
    scorekeeper TEXT, -- just keep the data since this could be anybody
    division_id INTEGER NOT NULL, -- FK tournament_division, could be a no-count or mixed divisions so we can't rely on the teams division, no count is always a division of '999'
    questions INTEGER NOT NULL DEFAULT 0, -- in case we want to publish real time scores, this is the current or final question number
    round_complete INTEGER NOT NULL DEFAULT 0, -- in case we want to publish real time scores
    ticker TEXT, -- contains the current state of the quiz as a single line, e.g. John called on to answer...John got question 10 correct...team 2 called a timeout... John prejumped
    results JSON, -- json containing each question, who prejumped and when, seat order, and all the details from the scoresheet
    FOREIGN KEY (room_id) REFERENCES tournament_room(id),
    FOREIGN KEY (quizmaster_id) REFERENCES registration_quizmaster(id),
    FOREIGN KEY (consultant_id) REFERENCES registration_quizmaster(id),
    FOREIGN KEY (division_id) REFERENCES tournament_division(id)
);

CREATE TABLE tournament_round_individuals_quizzers (
    id INTEGER NOT NULL PRIMARY KEY,
    individuals_id INTEGER NOT NULL, -- FK tournament_round_individuals
    quizzer_id INTEGER NOT NULL, -- FK registration_quizzer
    score INTEGER NOT NULL DEFAULT 0, -- in case we want to publish real time scores
    missed_prejumps INTEGER NOT NULL DEFAULT 0, -- in case we want to publish real time scores
    FOREIGN KEY (individuals_id) REFERENCES tournament_round_individuals(id),
    FOREIGN KEY (quizzer_id) REFERENCES registration_quizzer(id)
);
