use crate::player::Player;
use crate::team::Team;
use diesel::prelude::*;
use diesel::PgConnection;
use crate::history::{ NewPlayerRanking, NewTeamRanking };
use crate::game::Game;

const CHANGE_MULTIPLIER_K: f32 = 30.0;

pub fn update_rankings(
	connection: &PgConnection,
	winning_team: i32,
	losing_team: i32,
	game_id: i32
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

	update_player_rankings(connection, &winning_team, &losing_team, game_id)?;
	update_team_rankings(connection, &winning_team, &losing_team, game_id)?;

	Ok(())
}

fn update_player_rankings(
	connection: &PgConnection,
	winning_team: &Team,
	losing_team: &Team,
	game_id: i32
) -> Result<(), String> {
	
	let winner_one = Player::find(connection, winning_team.player_one_id)?;
	let winner_two = Player::find(connection, winning_team.player_two_id)?;

	let loser_one = Player::find(connection, losing_team.player_one_id)?;
	let loser_two = Player::find(connection, losing_team.player_two_id)?;

	let winners_rating = get_team_rating(winner_one.ranking, winner_two.ranking);
	let losers_rating = get_team_rating(loser_one.ranking, loser_two.ranking);

	let win_change = get_rating_change_win(winners_rating, losers_rating);
	let los_change = get_rating_change_loss(winners_rating, losers_rating);

	update_player_rank(connection, winner_one, win_change, game_id)?;
	update_player_rank(connection, winner_two, win_change, game_id)?;
	update_player_rank(connection, loser_one, los_change, game_id)?;
	update_player_rank(connection, loser_two, los_change, game_id)?;

	Ok(())
}

fn update_team_rankings(
	connection: &PgConnection,
	winning_team: &Team,
	losing_team: &Team,
	game_id: i32
) -> Result<(), String> {
	
	let winners_rating = winning_team.ranking;
	let losers_rating = losing_team.ranking;

	let win_change = get_rating_change_win(winners_rating, losers_rating);
	let los_change = get_rating_change_loss(losers_rating, winners_rating);

	update_team_rank(connection, winning_team, win_change, game_id)?;
	update_team_rank(connection, losing_team, los_change, game_id)?;

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

fn update_team_rank(connection: &PgConnection, team: &Team, change: i32, game_id: i32) -> Result<Team, String> {
	use crate::schema::teams::dsl as t;
	use crate::schema::team_rankings as tr;

	let new_rank = team.ranking + change;

	let team = diesel::update(t::teams.filter(t::id.eq(team.id)))
		.set(t::ranking.eq(new_rank))
		.get_result::<Team>(connection)
		.map_err(|e| format!("Couldn't update team ranking: {}", e))?;

	// TODO: record the rank change
	let new_team_rank = NewTeamRanking {
		team_id: team.id,
		current_ranking: new_rank,
		change,
		game_id
	};

	diesel::insert_into(tr::table)
		.values(&new_team_rank)
		.execute(connection)
		.map_err(|e| format!("Couldn't create new player ranking: {}", e))?;

	Ok(team)
}

fn update_player_rank(connection: &PgConnection, player: Player, change: i32, game_id: i32) -> Result<Player, String> {
	use crate::schema::players::dsl as p;
	use crate::schema::player_rankings as pr;

	let new_rank = player.ranking + change;

	let player = diesel::update(p::players.filter(p::id.eq(player.id)))
		.set(p::ranking.eq(new_rank))
		.get_result::<Player>(connection)
		.map_err(|e| format!("Couldn't update player ranking: {}", e))?;

	// TODO: record the rank change
	let new_player_rank = NewPlayerRanking {
		player_id: player.id,
		current_ranking: new_rank,
		change,
		game_id
	};

	diesel::insert_into(pr::table)
		.values(&new_player_rank)
		.execute(connection)
		.map_err(|e| format!("Couldn't create new player ranking: {}", e))?;

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

pub fn replay_ranking(connection: &PgConnection) -> Result<(), String>{
	use crate::schema::players::dsl as p;
	use crate::schema::teams::dsl as t;
	use crate::schema::player_rankings::dsl as pr;
	use crate::schema::team_rankings::dsl as tr;

	use crate::schema::games;

	diesel::update(p::players)
		.set(p::ranking.eq(1500))
		.execute(connection)
		.map_err(|e| format!("Couldn't update player ranking for replay: {}", e))?;
	
	diesel::update(t::teams)
		.set(t::ranking.eq(1500))
		.execute(connection)
		.map_err(|e| format!("Couldn't update player ranking for replay: {}", e))?;

	diesel::delete(pr::player_rankings)
		.execute(connection)
		.map_err(|e| format!("Couldn't delete player rankings for replay: {}", e))?;

	diesel::delete(tr::team_rankings)
		.execute(connection)
		.map_err(|e| format!("Couldn't delete team rankings for replay: {}", e))?;

	let results = games::table
		.order(games::dsl::id.asc())
		.load::<Game>(connection)
		.map_err(|e| format!("Load games: {}", e))?;
	
	for game in results {
		let losers = if game.winners.unwrap() == game.team_one_id {
			game.team_two_id
		} else {
			game.team_one_id
		};
		update_rankings(connection, game.winners.unwrap(), losers, game.id)?;
	}

	Ok(())
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
