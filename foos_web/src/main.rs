// Need this because musl and postgres combo
#[cfg(target_env = "musl")]
extern crate openssl;

use actix_cors::Cors;
use actix_files as files;
use actix_session::{CookieSession, Session};
use actix_web::{web, App, HttpServer};

mod config;
mod game;
mod player;
mod reports;
mod series;
mod user;

use config::Config;
use foos::database::*;

fn main() {
	let config = config::FoosWebConfig::from_toml_file("web-config.toml");
	let _ = config.save("web-config.toml");

	if config.database_url.trim().to_string().is_empty() {
		println!("Must specify a database-url in the web-config.toml file");
		return;
	}

	let connection_pool: ConnectionPool = match ConnectionPool::create(config.database_url) {
		Ok(cp) => cp,
		Err(e) => {
			println!("Could not create connection pool: {}", e);
			return;
		}
	};

	{
		// Want to make sure not to keep this db connection arround for the whole lifetime of the application.
		let db = match connection_pool.get() {
			Ok(db) => db,
			Err(_e) => {
				println!("Could not get the underlying database to run migrations");
				return;
			}
		};
		foos::run_pending_migrations(db.connection());
	}

	let secure_cookies = config.secure_cookies;
	HttpServer::new(move || {
		App::new()
			.wrap(
				Cors::new()
					//.allowed_origin("localhost:4200")
					.allowed_methods(vec!["GET", "POST", "PUT"])
					.supports_credentials(),
			)
			.wrap(
				CookieSession::private(&[0; 32])
					.secure(secure_cookies)
					.http_only(true),
			)
			.data(connection_pool.clone())
			// Users
			.route("/user/search/{term}", web::get().to(user::search))
			.route("/user/create", web::post().to(user::create))
			.route("/user/authenticate", web::post().to(user::authenticate))
			.route("/user/authenticated", web::get().to(user::authenticated))
			// Players
			.route(
				"player/search/{term}/{limit}",
				web::get().to(player::search),
			)
			.route("player/create", web::post().to(player::create))
			// Series
			.route("series/create", web::post().to(series::create))
			.route("gauntlet/create", web::post().to(series::create_gauntlet))
			// Record
			.route("game/finish", web::post().to(game::finish))
			// Reports
			.route(
				"report/leaderboard",
				web::get().to(reports::get_leader_board),
			)
			.route(
				"report/teamleaderboard",
				web::get().to(reports::get_team_leader_board),
			)
			.route(
				"report/playerstats/{player_id}",
				web::get().to(reports::get_player_stats),
			)
			.route(
				"report/playergames/{player_id}",
				web::get().to(reports::get_player_games),
			)
			.route(
				"report/teamstats/{player_one_id}/{player_two_id}",
				web::get().to(reports::get_team_stats),
			)
			// This service call needs to be at the end so all of the other routes get processed first.
			.service(
				files::Files::new("/", "./foosClient/dist/foosClient").index_file("index.html"),
			)
	})
	.bind(&config.bind_url)
	.expect(&format!("Can not bind to {}", &config.bind_url))
	.run()
	.unwrap();
}

pub fn get_session_user_id(session: Session) -> Result<i32, String> {
	let user_id = match session.get("user_id") {
		Ok(id) => id,
		Err(_e) => return Err("Couldn't get session".to_string()),
	};

	let user_id = match user_id {
		Some(id) => id,
		None => return Err("Couldn't get session".to_string()),
	};

	Ok(user_id)
}
