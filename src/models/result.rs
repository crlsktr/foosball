#![allow(dead_code)]
use super::super::schema::results;

#[derive(Queryable,  Deserialize)]
pub struct Result {
    pub game_id: i32,
    pub winning_team: i32,
    pub spread: i32
}

#[derive(Insertable, Deserialize)]
#[table_name="results"]
pub struct NewResult {
    pub game_id: i32,
    pub winning_team: i32,
    pub spread: i32
}