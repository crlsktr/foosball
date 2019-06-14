#![allow(dead_code)]
use super::super::schema::matches;

#[derive(Queryable, Deserialize)]
pub struct Match {
	pub id: i32,
	pub is_gauntlet: bool,
	pub date: String,
}

#[derive(Insertable, Deserialize)]
#[table_name = "matches"]
pub struct NewMatch {
	pub is_gauntlet: bool,
	pub date: String,
}
