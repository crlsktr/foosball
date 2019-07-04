use diesel::prelude::*;
use diesel::PgConnection;

const CHANGE_MULTIPLIER_K: f32 = 30.0;

pub fn update_rankings(connection: &PgConnection, winning_team: i32, losing_team: i32) Result<(), String> {
    let winning_team = Team::find(connection, winning_team)?;
    let losing_team = Team::find(connection, losing_team)?;

    let winner_one = Player::find(connection, winning_team.player_one_id)?;
    let winner_two = Player::find(connection, winning_team.player_two_id)?;

    let loser_one = Player::find(connection, losing_team.player_one_id)?;
    let loser_two = Player::find(connection, losing_team.player_two_id)?;

    let winners_rating = get_team_rating(winner_one.rating, winner_two.rating);
    let losers_rating = get_team_rating(loser_one.rating, loser_two.rating);

    let win_prob = probability_win(winners_rating, losers_rating);
    let los_prob = probability_win(losers_rating, winners_rating);

    let win_change = change_rank_win(win_prob);
    let los_change = change_rank_loss(los_prob);

    update_rank(connection, winner_one, win_change);
    update_rank(connection, winner_two, win_change);
    update_rank(connection, loser_one, los_change);
    update_rank(connection, loser_two, los_change);
}

fn update_rank(connection: &PgConnection, player: Player, change: i32) -> Result<Player, String> {
    use schema::players;
    use schema::players::dsl as p;

    let new_rank = player.ranking + change;

    let player = diesel::update(players::table)
			.set(p::ranking.eq(new_rank))
			.get_result::<Game>(connection)
			.map_err(|e| format!("Couldn't update game record: {}", e))?;

    Ok(player)
}


fn get_team_rating(rating_1: i32, rating_2: i32) -> i32 {
    (rating_1 + rating_2) / 2
}

fn probability_win(rating_1: i32, rating_2: i32) -> f32 {
    1.0 / ( 1.0 + (10.0f32).powf((rating_1 as f32 - rating_2 as f32) / 400.0))
}

fn change_rank_win(probability: f32) -> i32 {
    (CHANGE_MULTIPLIER_K * (1.0 - probability)).ceil() as i32;
}

fn change_rank_loss(probability: f32) -> i32 {
    (CHANGE_MULTIPLIER_K * (0.0 - probability)).ceil() as i32;
}