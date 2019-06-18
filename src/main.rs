#![allow(proc_macro_derive_resolution_fallback)]

#[macro_use]
extern crate rouille;
#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate diesel;
extern crate inflector;
#[macro_use]
extern crate tera;
extern crate chrono;

use tera::Tera;
use self::diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use std::sync::Mutex;

pub mod models;
pub mod schema;
pub mod game_results;
pub mod play;
pub mod create_match;
pub mod index;
pub mod leaderboard;
pub mod playerstats;
pub mod shared_queries;
pub mod teamstats;


fn main() {
	let db = {
		let db = SqliteConnection::establish("./foosball.db");
		// I don't think this fails with maybe the exception of permisisons.
		Mutex::new(db.expect("Failed to connect to ./foosball.db"))
	};

	let templates: Tera = compile_templates!("templates/**/*");

	rouille::start_server("0.0.0.0:12346", move |request| {
		// This shouldn't happen because rouille is single threaded.
		let connection = db.lock().expect("database in use");
		router!(request,
			(POST) (/results) => {
				game_results::save_game_results(request, &*connection)
			},

			(POST) (/play) => {
				play::play_match(request, &*connection, &templates)
			},

			(GET) (/) => {
				index::get_index(request, &*connection, &templates)
			},

			(GET) (/new/{match_type: String}) => {
				create_match::new_match(request, &*connection, match_type, &templates)
			},

			(GET) (/leaderboard) => {
				leaderboard::get_leaders(request, &*connection, &templates)
			},

            (GET) (/playerstats) => {
               playerstats::get_player_stats(request, &*connection, &templates)
            },

			(GET) (/teamstats) => {
                teamstats::get_team_stats(request, &*connection, &templates)
			},
			_ => rouille::Response::empty_404()
		)
	});
}