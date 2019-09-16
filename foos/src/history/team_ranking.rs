use crate::schema::team_rankings;

#[derive(Insertable)]
#[table_name = "team_rankings"]
pub struct NewTeamRanking {
	pub team_id: i32,
	pub current_ranking: i32,
    pub change: i32,
    pub game_id: i32,
}