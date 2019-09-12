-- Your SQL goes here
CREATE TABLE ranking_histories 
(
	id SERIAL PRIMARY KEY NOT NULL,
	player_id INTEGER NOT NULL,
	FOREIGN KEY (player_one_id) REFERENCES players (id)
)