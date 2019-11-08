use actix_session::Session;
use serde_derive::{Deserialize, Serialize};

use actix_web::{web, Responder};

use foos::database::*;
use foos::player::Player;

pub fn search(path: web::Path<(String, i64)>, pool: web::Data<ConnectionPool>) -> impl Responder {
	let db = match pool.get() {
		Ok(db) => db,
		Err(e) => return web::Json(Err(format!("Couldn't get the database: {}", e))),
	};
	let term = if path.0 == "all" {
		"".to_string()
	} else {
		path.0.clone()
	};
	let response = Player::search(db.connection(), &term, path.1);
	web::Json(response)
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreatePlayerRequest {
	user_id: Option<i32>,
	name: String,
}

pub fn create(
	request: web::Json<CreatePlayerRequest>,
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
	let response = Player::create(db.connection(), request.user_id, &request.name, user_id);
	web::Json(response)
}
