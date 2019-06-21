#![allow(proc_macro_derive_resolution_fallback)]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;
extern crate diesel;
extern crate inflector;
extern crate chrono;
extern crate foos;

use diesel::sqlite::SqliteConnection;
use std::sync::Mutex;

use foos::rank::*;
use foos::schema;
use foos::models::*;
use foos::shared_queries::*;


fn main() {

    let db = {
		let db = SqliteConnection::establish("./foosball.db");
		// I don't think this fails with maybe the exception of permisisons.
		Mutex::new(db.expect("Failed to connect to ./foosball.db"))
	};

    // Do I care about this?
    let connection = db.lock().expect("database in use");

    // load all of the matches in order of match played
    let matches = schema::matches::dsl::matches
        .order_by(schema::matches::date.asc())
        .load::<Match>(&*connection)
        .expect("Yikes");

    let matches: Vec<i32> = matches.iter().map(|m| m.id).collect();

    //dbg!(&matches);

    for match_id in matches {
        let games = schema::games::dsl::games
        .filter(schema::games::dsl::match_id.eq(match_id))
        .load::<Game>(&*connection)
        .expect("Couldn't Load games for match");

        for game in  games {
            let game_result = schema::results::dsl::results
            .filter(schema::results::dsl::game_id.eq(game.id))
            .first::<Result>(&*connection);
            
            let _ = match game_result {
                Ok(r) => r,
                Err(_e) => {
                    println!("Couldn't load result for game {}", game.id);
                    continue;
                }
            };

            //get the winning and losing teams 
            let mut win_team : Vec<Player> =  
                diesel::sql_query(game_win_team(game.id))
                .load(&*connection)
                .unwrap_or(vec!());
            
            let mut los_team: Vec<Player> = 
                diesel::sql_query(game_los_team(game.id))
                .load(&*connection)
                .unwrap_or(vec!());

            if win_team.len() < 2 {
                println!("didn't load winning players");
                continue;
            }

            if los_team.len() < 2 {
                println!("didn't load losing players");
                continue;
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
            let results = update_rank(w, &*connection);
            match results {
                Ok(_) => println!("updated."),
                Err(_) => println!("failed to update rank")
            };
        }
    }
    
    // get all of the games from the matches in order of game played

    // update rank for each game/player
}