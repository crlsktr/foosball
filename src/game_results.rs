use crate::schema;
use crate::models::*;
use diesel::sqlite::SqliteConnection;
use diesel::RunQueryDsl;

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

    Ok(())
}