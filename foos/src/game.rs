use crate::schema::games;
use crate::ranking::update_rankings;
use diesel::prelude::*;
use diesel::PgConnection;

#[derive(Queryable, QueryableByName, Serialize, Debug, Clone)]
#[table_name = "games"]
pub struct Game {
	pub id: i32,
	pub series_id: i32,
	pub team_one_id: i32,
	pub team_two_id: i32,
	pub winners: Option<i32>,
	pub spread: Option<i16>,
	pub recorded_by: Option<i32>,
}

#[derive(Insertable, Deserialize)]
#[table_name = "games"]
pub struct NewGame {
	pub series_id: i32,
	pub team_one_id: i32,
	pub team_two_id: i32,
	pub winners: Option<i32>,
	pub spread: Option<i16>,
	pub recorded_by: Option<i32>,
}

impl Game {
	pub fn create(
		connection: &PgConnection,
		series_id: i32,
		team_one_id: i32,
		team_two_id: i32,
	) -> Result<Game, String> {
		let new_game = NewGame {
			series_id,
			team_one_id,
			team_two_id,
			winners: None,
			spread: None,
			recorded_by: None,
		};
		let game = diesel::insert_into(games::table)
			.values(&new_game)
			.get_result(connection)
			.map_err(|e| format!("Couldn't create game: {}", e))?;

		Ok(game)
	}

	pub fn find(connection: &PgConnection, id: i32) -> Result<Game, String> {
		use games::dsl as p;
		let game: Game = p::games
			.find(id)
			.first::<Game>(connection)
			.map_err(|e| format!("Unable to find game: {}", e))?;
		Ok(game)
	}

	pub fn finish(
		&mut self,
		connection: &PgConnection,
		spread: i16,
		winners: i32,
		recorded_by: i32,
	) -> Result<(), String> {
		use games::dsl as g;
		if spread > 10 || spread < 1 {
			return Err(format!(
				"A spread of {} is not in the value range of 1-10",
				spread
			));
		}
		if winners != self.team_one_id && winners != self.team_two_id {
			return Err(format!(
				"Couldn't set winners to team {} for game {}: That team didn't play in this game",
				self.id, winners
			));
		}
		if self.spread.is_some() && self.winners.is_some() {
			return Err(format!("Game {}  has already been recorded", self.id));
		}
		let game = diesel::update(g::games.find(self.id))
			.set((
				g::winners.eq(Some(winners)),
				g::recorded_by.eq(Some(recorded_by)),
				g::spread.eq(Some(spread)),
			))
			.get_result::<Game>(connection)
			.map_err(|e| format!("Couldn't update game record: {}", e))?;
		self.spread = game.spread;
		self.winners = game.winners;
		let losers = if game.winners.unwrap() == game.team_one_id { game.team_two_id } else { game.team_one_id }; 
		update_rankings(connection, game.winners.unwrap(), losers)?;

		Ok(())
	}
}
