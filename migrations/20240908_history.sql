CREATE TABLE history (
    id INTEGER NOT NULL PRIMARY KEY,
    year INTEGER NOT NULL,

    finals_location TEXT NOT NULL,
    finals_city TEXT,
    finals_state TEXT,

    material TEXT NOT NULL,
    material_translation TEXT NOT NULL,

    markell_recipient TEXT NOT NULL,
    markell_recipient_city TEXT NOT NULL,
    markell_recipient_state TEXT NOT NULL,

    alpha_omega_winner_church TEXT NOT NULL,
    alpha_omega_winner_city TEXT NOT NULL,
    alpha_omega_winner_state TEXT NOT NULL,
    alpha_omega_winner_team_members TEXT NOT NULL,

    stv_winner TEXT NOT NULL,

    notes TEXT NOT NULL
);

CREATE TABLE hall_of_fame (
    id INTEGER NOT NULL PRIMARY KEY,
    inductee TEXT NOT NULL,
    home_church TEXT NOT NULL,
    home_church_city TEXT,
    home_church_state TEXT,
    years_quizzed TEXT,
    induction_year INTEGER NOT NULL,
    bio TEXT,
    image BLOB
);

CREATE TABLE hall_of_fame_qa (
    id INTEGER NOT NULL PRIMARY KEY,
    hall_of_fame_id INTEGER NOT NULL, 
    question TEXT NOT NULL,
    answer TEXT NOT NULL
);
