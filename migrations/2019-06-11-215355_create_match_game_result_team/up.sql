-- Your SQL goes here

-- We already had a games but we didn't like it.
DROP TABLE games;

CREATE TABLE matches (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    is_gauntlet BOOLEAN NOT NULL,
    date VARCHAR NOT NULL
);

CREATE TABLE teams (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    player_one_id INTEGER NOT NULL,
    player_two_id INTEGER NOT NULL,
    CONSTRAINT fk_player_one_player
        FOREIGN KEY (player_one_id)
        REFERENCES players (id)
    CONSTRAINT fk_player_two_player
        FOREIGN KEY (player_two_id)
        REFERENCES players (id)
);

CREATE TABLE games (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    match_id INTEGER NOT NULL,
    team_one_id INTEGER NOT NULL,
    team_two_id INTEGER NOT NULL,
    CONSTRAINT fk_match_id_matchs
        FOREIGN KEY (match_id)
        REFERENCES matches (id)
    CONSTRAINT fk_team_one_id_teams
        FOREIGN KEY (team_one_id)
        REFERENCES teams (id)
    CONSTRAINT fk_team_two_id_teams
        FOREIGN KEY (team_two_id)
        REFERENCES teams (id)
);

CREATE TABLE results (
    game_id INTEGER PRIMARY KEY NOT NULL,
    winning_team INTEGER NOT NULL,
    spread INTEGER NOT NULL,
    CONSTRAINT fk_game_id_games
        FOREIGN KEY (game_id)
        REFERENCES games (id)
);