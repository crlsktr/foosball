use diesel::sqlite::SqliteConnection;
use diesel::prelude::*;
use diesel::RunQueryDsl;

const CHANGE_MULTIPLIER_K: f32 = 30.0;

/// Updates the rank of 
pub fn update_rank(newrankings: Vec<(i32, i32)>, connection: &SqliteConnection ) -> std::result::Result<(), String> {
	use schema::players::dsl::*;
	for single_rank in newrankings
	{
		diesel::update(players.filter(id.eq(single_rank.0)))
		.set(ranking.eq(single_rank.1))
		.execute(connection)
		.map_err(|_| format!("Couldn't update the ranking for {}.", single_rank.0))?;
	}
	Ok(())
}

pub fn change_rank_win(probability: f32) -> f32{
    CHANGE_MULTIPLIER_K * (1.0 - probability)
}

pub fn change_rank_loss(probability: f32) -> f32 {
    CHANGE_MULTIPLIER_K * (0.0 - probability)
}

pub fn probability_win(rating_1: f32, rating_2: f32) -> f32 {
	1.0 / ( 1.0 + (10.0f32).powf((rating_2 - rating_1) / 400.0))
}