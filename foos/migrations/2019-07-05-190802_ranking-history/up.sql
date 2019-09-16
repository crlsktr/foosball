-- Your SQL goes here
CREATE TABLE player_rankings
(
	id SERIAL PRIMARY KEY NOT NULL,
	player_id INTEGER NOT NULL,
	current_ranking INTEGER NOT NULL,
	change INTEGER NOT NULL,
	game_id INTEGER NOT NULL,
	FOREIGN KEY (player_id) REFERENCES players (id),
	FOREIGN KEY (game_id) REFERENCES games (id)
);

CREATE TABLE team_rankings
(
	id SERIAL PRIMARY KEY NOT NULL,
	team_id INTEGER NOT NULL,
	current_ranking INTEGER NOT NULL,
	change INTEGER NOT NULL,
	game_id INTEGER NOT NULL,
	FOREIGN KEY (team_id) REFERENCES teams (id),
	FOREIGN KEY (game_id) REFERENCES games (id)
);