use actix_web::{web, Responder};

use foos;
use foos::database::*;

pub fn get_leader_board(pool: web::Data<ConnectionPool>) -> impl Responder {
	let db = match pool.get() {
		Ok(db) => db,
		Err(e) => return web::Json(Err(format!("Couldn't get the database: {}", e))),
	};
	let response = foos::reports::leader_board(db.connection());
	web::Json(response)
}

pub fn get_team_leader_board(pool: web::Data<ConnectionPool>) -> impl Responder {
	let db = match pool.get() {
		Ok(db) => db,
		Err(e) => return web::Json(Err(format!("Couldn't get the database: {}", e))),
	};
	let response = foos::reports::team_leader_board(db.connection());
	web::Json(response)
}

pub fn get_player_stats(pool: web::Data<ConnectionPool>, path: web::Path<i32>) -> impl Responder {
	let db = match pool.get() {
		Ok(db) => db,
		Err(e) => return web::Json(Err(format!("Couldn't get the database: {}", e))),
	};
	let player_id: i32 = *path;
	let response = foos::reports::player_stats(db.connection(), player_id);
	web::Json(response)
}

pub fn get_player_games(pool: web::Data<ConnectionPool>, path: web::Path<i32>) -> impl Responder {
	let db = match pool.get() {
		Ok(db) => db,
		Err(e) => return web::Json(Err(format!("Couldn't get the database: {}", e))),
	};
	let player_id: i32 = *path;
	let response = foos::reports::player_games(db.connection(), player_id);
	web::Json(response)
}

pub fn get_team_stats(
	pool: web::Data<ConnectionPool>,
	path: web::Path<(i32, i32)>,
) -> impl Responder {
	let db = match pool.get() {
		Ok(db) => db,
		Err(e) => return web::Json(Err(format!("Couldn't get the database: {}", e))),
	};
	let player_one_id: i32 = path.0;
	let player_two_id: i32 = path.1;
	let response = foos::reports::team_stats(db.connection(), player_one_id, player_two_id);
	web::Json(response)
}

pub fn get_team_games(
	pool: web::Data<ConnectionPool>,
	path: web::Path<i32>,
) -> impl Responder {
	let db = match pool.get() {
		Ok(db) => db,
		Err(e) => return web::Json(Err(format!("Couldn't get the database: {}", e))),
	};

	let team_id: i32 = *path;
	let response = foos::reports::team_games(db.connection(), team_id);
	web::Json(response)
}
