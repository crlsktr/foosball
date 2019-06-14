#![allow(dead_code)]
use super::super::schema::games;
use super::Team;

#[derive(Queryable, Serialize)]
pub struct Game {
	pub id: i32,
	pub match_id: i32,
	pub team_one_id: i32,
	pub team_two_id: i32,
}

#[derive(Insertable, Deserialize)]
#[table_name = "games"]
pub struct NewGame {
	pub match_id: i32,
	pub team_one_id: i32,
	pub team_two_id: i32,
}

pub struct TeamGame {
	pub id: i32,
	pub match_id: i32,
	pub team_one_id: i32,
	pub team_one: Team,
	pub team_two_id: i32,
	pub team_two: Team,
}
