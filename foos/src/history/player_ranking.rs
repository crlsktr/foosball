use crate::schema::player_rankings;

#[derive(Insertable)]
#[table_name = "player_rankings"]
pub struct NewPlayerRanking {
	pub player_id: i32,
	pub current_ranking: i32,
    pub change: i32,
    pub game_id: i32,
}