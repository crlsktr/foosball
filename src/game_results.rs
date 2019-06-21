use crate::schema;
use crate::models::*;
use diesel::sqlite::SqliteConnection;
use diesel::prelude::*;
use diesel::RunQueryDsl;
use crate::rank::*;
use crate::shared_queries::*;

pub fn save_game_results(request: &rouille::Request, connection: &SqliteConnection) -> rouille::Response {
    let r = try_or_400!(post_input!(request, {
        game_0: i32,
        spread_0: i32,
        winner_0: i32,

        game_1: i32,
        spread_1: i32,
        winner_1: i32,

        game_2: i32,
        spread_2: i32,
        winner_2: i32,

        game_3: Option<i32>,
        spread_3: Option<i32>,
        winner_3: Option<i32>,

        game_4: Option<i32>,
        spread_4: Option<i32>,
        winner_4: Option<i32>,
    }));

    match create_results(r.game_0, r.spread_0, r.winner_0, &*connection) {
        Ok(_) => {},
        Err(e) => return rouille::Response::text(format!("Well that sucks {}", e)).with_status_code(500)
    }

    match create_results(r.game_1, r.spread_1, r.winner_1, &*connection)  {
        Ok(_) => {},
        Err(e) => return rouille::Response::text(format!("Well that sucks {}", e)).with_status_code(500)
    }

    match create_results(r.game_2, r.spread_2, r.winner_2, &*connection) {
        Ok(_) => {},
        Err(e) => return rouille::Response::text(format!("Well that sucks {}", e)).with_status_code(500)
    }
    
    if r.game_3.is_some() && r.spread_3.is_some() && r.winner_3.is_some() {
        match create_results(r.game_3.unwrap(), r.spread_3.unwrap(), r.winner_3.unwrap(), &*connection) {
            Ok(_) => {},
            Err(e) => return rouille::Response::text(format!("Well that sucks {}", e)).with_status_code(500)
        }
    }
    
    if r.game_4.is_some() && r.spread_4.is_some() && r.winner_4.is_some() {
        match create_results(r.game_4.unwrap(), r.spread_4.unwrap(), r.winner_4.unwrap(), &*connection) {
            Ok(_) => {},
            Err(e) => return rouille::Response::text(format!("Well that sucks {}", e)).with_status_code(500)
        }
    }

    rouille::Response::html("Results Saved")
}

fn create_results(game_id: i32, spread: i32, winning_team_id: i32, connection: &SqliteConnection) -> std::result::Result<(), String> {
	let game_result = NewResult {
		game_id,
		winning_team: winning_team_id,
		spread,
	};
	
	diesel::insert_into(schema::results::table)
		.values(&game_result)
		.execute(connection)
        .map_err(|_| format!("Couldn't insert {:?} into game results.", game_result))?;
	
	//get the winning and losing teams 
	let win_team_query = game_win_team(game_id);
	let mut win_team : Vec<Player> =  
		diesel::sql_query(win_team_query)
		.load(&*connection)
		.unwrap_or(vec!());
	
	let mut los_team: Vec<Player> = 
		diesel::sql_query(game_los_team(game_id))
		.load(&*connection)
		.unwrap_or(vec!());

    if win_team.len() < 2 {
        return Err("Couldn't load the winning team".to_string());
    }

    if los_team.len() < 2 {
        return Err("Couldn't load the losing team".to_string());
    }

	let winner_rating = win_team[0].ranking + win_team[1].ranking;
	let loser_rating = los_team[0].ranking + los_team[1].ranking;

	let win_prob = probability_win(winner_rating as f32, loser_rating as f32);
	let los_prob = probability_win(loser_rating as f32, winner_rating as f32);

	let win_change = change_rank_win(win_prob); 
	let los_change = change_rank_loss(los_prob);

	win_team[0].ranking += (win_change).ceil() as i32;
	win_team[1].ranking += (win_change).ceil() as i32;

	los_team[0].ranking += (los_change).ceil() as i32;
	los_team[1].ranking += (los_change).ceil() as i32;

	let mut w :Vec<(i32,i32)> = win_team.iter().map(|t| (t.id, t.ranking)).collect();
	let mut l :Vec<(i32,i32)> = los_team.iter().map(|t| (t.id, t.ranking)).collect();

	w.append(& mut l);
	update_rank(w, connection)?;
	//Ra = Ra + K * (1 - Pa); 
    //Rb = Rb + K * (0 - Pb); 
	//win_team.player[s].ranking += win_team_win_prob(los_team) + K * (1 - win_team_prob)
	//los_team.player[s].ranking += los_team_win_prob(win_team) + K * (0 - los_team_prob)
	//save all players

    Ok(())
}