-- Your SQL goes here
CREATE TABLE ranking_histories 
(
	id SERIAL PRIMARY KEY NOT NULL,
	player_id INTEGER NOT NULL,
	current_ranking INTEGER NOT NULL,
	change INTEGER NOT NULL,
	game_id INTEGER NOT NULL,
	FOREIGN KEY (player_id) REFERENCES players (id),
	FOREIGN KEY (game_id) REFERENCES games (id)
)