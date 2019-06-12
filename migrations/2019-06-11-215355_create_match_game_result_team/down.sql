-- This file should undo anything in `up.sql`
DROP TABLE results;
DROP TABLE games;
DROP TABLE teams;
DROP TABLE matches;

CREATE TABLE games (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    round VARCHAR NOT NULL,
    date VARCHAR NOT NULL,
    teamone VARCHAR NOT NULL,
    teamtwo VARCHAR NOT NULL,
    scoreone INTEGER NOT NULL,
    scoretwo INTEGER NOT NULL
)