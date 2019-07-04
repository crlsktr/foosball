use crate::schema::players;
use diesel::prelude::*;
use diesel::PgConnection;

#[derive(Queryable, QueryableByName, Serialize, Deserialize, Debug, Clone)]
#[table_name = "players"]
pub struct Player {
	pub id: i32,
	pub user_id: Option<i32>,
	pub name: String,
	pub ranking: i32,
}

#[derive(Insertable, Deserialize)]
#[table_name = "players"]
struct NewPlayer {
	pub user_id: Option<i32>,
	pub name: String,
	pub ranking: i32,
}

impl Player {
	pub fn create<S: ToString>(
		connection: &PgConnection,
		user_id: Option<i32>,
		name: S,
	) -> Result<Player, String> {
		let new_player = NewPlayer {
			user_id,
			name: name.to_string(),
			ranking: 1500,
		};

		let player = diesel::insert_into(players::table)
			.values(&new_player)
			.get_result(connection)
			.map_err(|e| format!("Couldn't create player: {}", e))?;

		Ok(player)
	}

	pub fn find(connection: &PgConnection, id: i32) -> Result<Player, String> {
		use players::dsl as p;
		let player: Player = p::players
			.find(id)
			.first::<Player>(connection)
			.map_err(|e| format!("Unable to find player: {}", e))?;
		Ok(player)
	}

	pub fn search(
		connection: &PgConnection,
		search: &str,
		limit: i64,
	) -> Result<Vec<Player>, String> {
		use crate::schema::players::dsl as p;
		let search = format!("%{}%", search);
		let players_ = p::players
			.filter(p::name.like(&search))
			.limit(limit)
			.load::<Player>(connection)
			.map_err(|e| format!("Unable to search for player: {}", e))?;
		Ok(players_)
	}
}
