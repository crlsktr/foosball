use actix_session::Session;
use serde_derive::{Deserialize, Serialize};

use actix_web::{web, Responder};

use foos;
use foos::database::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct RecordGamesRequest {
	game_results: Vec<foos::GameResult>,
}

pub fn finish(
	request: web::Json<RecordGamesRequest>,
	pool: web::Data<ConnectionPool>,
	session: Session,
) -> impl Responder {
	let db = match pool.get() {
		Ok(db) => db,
		Err(e) => return web::Json(Err(format!("Couldn't get the database: {}", e))),
	};
	let request = request.into_inner();
	let user_id = match crate::get_session_user_id(session) {
		Ok(u) => u,
		Err(e) => return web::Json(Err(e)),
	};
	match foos::finish_games(db.connection(), &request.game_results, user_id) {
		Ok(_) => web::Json(Ok("success")),
		Err(e) => web::Json(Err(e)),
	}
}
