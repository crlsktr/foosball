-- Your SQL goes here
CREATE TABLE users (
    id SERIAL PRIMARY KEY NOT NULL,
    username VARCHAR NOT NULL UNIQUE,
    password VARCHAR NOT NULL,
    enabled BOOLEAN NOT NULL DEFAULT 'f'
);


CREATE TABLE players (
    id SERIAL PRIMARY KEY NOT NULL,
    user_id INTEGER NULL,
    name VARCHAR NOT NULL UNIQUE,
    ranking INTEGER NOT NULL,
    FOREIGN KEY (user_id) REFERENCES users (id)
);

CREATE TABLE teams (
    id SERIAL PRIMARY KEY NOT NULL,
    player_one_id INTEGER NOT NULL,
    player_two_id INTEGER NOT NULL,
    ranking INTEGER NOT NULL,
    FOREIGN KEY (player_one_id) REFERENCES players (id),
    FOREIGN KEY (player_two_id) REFERENCES players (id)
);

CREATE TABLE series (
    id SERIAL PRIMARY KEY NOT NULL,
    played_on TIMESTAMP WITH TIME ZONE NOT NULL
);

CREATE TABLE games (
    id SERIAL PRIMARY KEY NOT NULL,
    series_id INTEGER NOT NULL,
    team_one_id INTEGER NOT NULL,
    team_two_id INTEGER NOT NULL,
    winners INTEGER NULL,
    spread SMALLINT NULL,
    FOREIGN KEY (series_id) REFERENCES series (id),
    FOREIGN KEY (team_one_id) REFERENCES teams (id),
    FOREIGN KEY (team_two_id) REFERENCES teams (id)
);