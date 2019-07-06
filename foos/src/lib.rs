#[macro_use]
extern crate diesel;
#[macro_use]
extern crate serde_derive;
extern crate serde;

use diesel::PgConnection;

pub mod database;
pub mod player;
pub mod reports;
pub mod user;

mod game;
mod schema;
mod series;
mod team;

use player::*;

#[derive(Serialize, Deserialize)]
pub struct Series {
	pub id: i32,
	pub games: Vec<Game>,
}

#[derive(Serialize, Deserialize)]
pub struct Game {
	pub id: i32,
	pub team_one: Team,
	pub team_two: Team,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Team {
	pub id: i32,
	pub player_one: Player,
	pub player_two: Player,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GameResult {
	pub id: i32,
	pub winners: i32,
	pub spread: i16,
}

pub fn create_series(
	connection: &PgConnection,
	players: [i32; 4],
	created_by: i32,
) -> Result<Series, String> {
	let players = find_players(connection, &players)?;

	let t1 = team::Team::create(connection, players[0].id, players[1].id)?;
	let t2 = team::Team::create(connection, players[0].id, players[2].id)?;
	let t3 = team::Team::create(connection, players[0].id, players[3].id)?;
	let t4 = team::Team::create(connection, players[1].id, players[2].id)?;
	let t5 = team::Team::create(connection, players[1].id, players[3].id)?;
	let t6 = team::Team::create(connection, players[2].id, players[3].id)?;

	let teams = vec![&t1, &t2, &t3, &t4, &t5, &t6];
	let teams: Vec<Team> = teams
		.iter()
		.map(|t| Team {
			id: t.id,
			player_one: players
				.iter()
				.find(|p| p.id == t.player_one_id)
				.unwrap()
				.clone(),
			player_two: players
				.iter()
				.find(|p| p.id == t.player_two_id)
				.unwrap()
				.clone(),
		})
		.collect();

	let series = series::Series::create(connection, created_by)?;

	let g1 = game::Game::create(connection, series.id, t1.id, t6.id)?;
	let g2 = game::Game::create(connection, series.id, t2.id, t5.id)?;
	let g3 = game::Game::create(connection, series.id, t3.id, t4.id)?;

	let games = vec![&g1, &g2, &g3];
	let games: Vec<Game> = games
		.iter()
		.map(|g| Game {
			id: g.id,
			team_one: teams
				.iter()
				.find(|t| t.id == g.team_one_id)
				.unwrap()
				.clone(),
			team_two: teams
				.iter()
				.find(|t| t.id == g.team_two_id)
				.unwrap()
				.clone(),
		})
		.collect();

	let series = Series {
		id: series.id,
		games,
	};
	Ok(series)
}

pub fn create_gauntlet(
	connection: &PgConnection,
	players: [i32; 5],
	created_by: i32,
) -> Result<Series, String> {
	let players = find_players(connection, &players)?;

	let t1 = team::Team::create(connection, players[0].id, players[1].id)?;
	let t2 = team::Team::create(connection, players[0].id, players[2].id)?;
	let t3 = team::Team::create(connection, players[0].id, players[3].id)?;
	let t4 = team::Team::create(connection, players[0].id, players[4].id)?;
	let t5 = team::Team::create(connection, players[1].id, players[2].id)?;
	let t6 = team::Team::create(connection, players[1].id, players[3].id)?;
	let t7 = team::Team::create(connection, players[1].id, players[4].id)?;
	let t8 = team::Team::create(connection, players[2].id, players[3].id)?;
	let t9 = team::Team::create(connection, players[2].id, players[4].id)?;
	let t10 = team::Team::create(connection, players[3].id, players[4].id)?;

	let teams = vec![&t1, &t2, &t3, &t4, &t5, &t6, &t7, &t8, &t9, &t10];
	let teams: Vec<Team> = teams
		.iter()
		.map(|t| Team {
			id: t.id,
			player_one: players
				.iter()
				.find(|p| p.id == t.player_one_id)
				.unwrap()
				.clone(),
			player_two: players
				.iter()
				.find(|p| p.id == t.player_two_id)
				.unwrap()
				.clone(),
		})
		.collect();

	let series = series::Series::create(connection, created_by)?;

	// game order is important here as this makes it match what is on the gauntlet whiteboard.
	let g1 = game::Game::create(connection, series.id, t5.id, t10.id)?;
	let g2 = game::Game::create(connection, series.id, t8.id, t4.id)?;
	let g3 = game::Game::create(connection, series.id, t3.id, t7.id)?;
	let g4 = game::Game::create(connection, series.id, t1.id, t9.id)?;
	let g5 = game::Game::create(connection, series.id, t6.id, t2.id)?;

	let games = vec![&g1, &g2, &g3, &g4, &g5];
	let games: Vec<Game> = games
		.iter()
		.map(|g| Game {
			id: g.id,
			team_one: teams
				.iter()
				.find(|t| t.id == g.team_one_id)
				.unwrap()
				.clone(),
			team_two: teams
				.iter()
				.find(|t| t.id == g.team_two_id)
				.unwrap()
				.clone(),
		})
		.collect();

	let series = Series {
		id: series.id,
		games,
	};
	Ok(series)
}

pub fn finish_games(
	connection: &PgConnection,
	game_results: &[GameResult],
	recorded_by: i32,
) -> Result<(), String> {
	let mut games = vec![];
	for result in game_results.iter() {
		let game = game::Game::find(connection, result.id)?;
		games.push((result.clone(), game));
	}

	for (result, mut game) in games {
		game.finish(connection, result.spread, result.winners, recorded_by)?;
	}

	Ok(())
}

fn find_players(connection: &PgConnection, players: &[i32]) -> Result<Vec<Player>, String> {
	let mut found = vec![];
	for id in players {
		let player = Player::find(connection, *id)?;
		found.push(player);
	}
	Ok(found)
}
