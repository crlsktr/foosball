-- Your SQL goes here
CREATE TABLE games (
    number INT,
    round VARCHAR,
    date VARCHAR,
    teamone VARCHAR,
    teamtwo VARCHAR,
    scoreone INT,
    scoretwo INT,
    PRIMARY KEY (number, round)
)