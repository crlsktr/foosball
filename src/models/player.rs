#![allow(dead_code)]
use super::super::schema::players;

#[derive(Queryable, Serialize, Debug,Clone)]
pub struct Player {
	pub id: i32,
	pub name: String,
}

#[derive(Insertable, Deserialize)]
#[table_name = "players"]
pub struct NewPlayer {
	pub name: String,
}
