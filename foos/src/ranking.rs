use crate::player::Player;
use crate::team::Team;
use diesel::prelude::*;
use diesel::PgConnection;

const CHANGE_MULTIPLIER_K: f32 = 30.0;

pub fn update_rankings(
	connection: &PgConnection,
	winning_team: i32,
	losing_team: i32,
) -> Result<(), String> {
	use crate::schema::teams::dsl as t;
	let winning_team = t::teams
		.filter(t::id.eq(winning_team))
		.first::<Team>(connection)
		.map_err(|e| format!("Unable to find winning team: {}", e))?;
	let losing_team = t::teams
		.filter(t::id.eq(losing_team))
		.first::<Team>(connection)
		.map_err(|e| format!("Unable to find losing team: {}", e))?;

	let winner_one = Player::find(connection, winning_team.player_one_id)?;
	let winner_two = Player::find(connection, winning_team.player_two_id)?;

	let loser_one = Player::find(connection, losing_team.player_one_id)?;
	let loser_two = Player::find(connection, losing_team.player_two_id)?;

	let winners_rating = get_team_rating(winner_one.ranking, winner_two.ranking);
	let losers_rating = get_team_rating(loser_one.ranking, loser_two.ranking);

	let win_change = get_rating_change_win(winners_rating, losers_rating);
	let los_change = get_rating_change_loss(losers_rating, winners_rating);

	update_rank(connection, winner_one, win_change)?;
	update_rank(connection, winner_two, win_change)?;
	update_rank(connection, loser_one, los_change)?;
	update_rank(connection, loser_two, los_change)?;

	Ok(())
}

pub fn get_rating_change_win(winner_rating: i32, loser_rating: i32) -> i32 {
	let win_prob = probability_win(winner_rating, loser_rating);
	change_rank_win(win_prob)
}

pub fn get_rating_change_loss(winner_rating: i32, loser_rating: i32) -> i32 {
	let win_prob = probability_win(loser_rating, winner_rating);
	change_rank_loss(win_prob)
}

fn update_rank(connection: &PgConnection, player: Player, change: i32) -> Result<Player, String> {
	use crate::schema::players::dsl as p;

	let new_rank = player.ranking + change;

	let player = diesel::update(p::players.filter(p::id.eq(player.id)))
		.set(p::ranking.eq(new_rank))
		.get_result::<Player>(connection)
		.map_err(|e| format!("Couldn't update game record: {}", e))?;

	Ok(player)
}

fn get_team_rating(rating_1: i32, rating_2: i32) -> i32 {
	(rating_1 + rating_2)
}

fn probability_win(rating_1: i32, rating_2: i32) -> f32 {
	1.0 / (1.0 + (10.0f32).powf((rating_2 as f32 - rating_1 as f32) / 400.0))
}

fn change_rank_win(probability: f32) -> i32 {
	(CHANGE_MULTIPLIER_K * (1.0 - probability)).ceil() as i32
}

fn change_rank_loss(probability: f32) -> i32 {
	(CHANGE_MULTIPLIER_K * (0.0 - probability)).ceil() as i32
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn rate_change_on_1500_vs_1500_win() {
		let change = get_rating_change_win(1500, 1500);
		assert_eq!(15, change);
	}

	#[test]
	fn rate_change_on_1500_vs_1500_loss() {
		let change = get_rating_change_loss(1500, 1500);
		assert_eq!(-15, change);
	}
}
