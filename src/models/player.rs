#![allow(dead_code)]
use super::super::schema::players;
use diesel::sql_types::{ Integer, Varchar};

#[derive(Queryable, QueryableByName, Serialize, Debug,Clone)]
pub struct Player {
	#[sql_type="Integer"]
	pub id: i32,
	#[sql_type="Varchar"]
	pub name: String,
	#[sql_type="Integer"]
	pub ranking: i32,
}

#[derive(Insertable, Deserialize)]
#[table_name = "players"]
pub struct NewPlayer {
	pub name: String,
}
