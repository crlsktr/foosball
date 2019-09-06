use actix_session::Session;
use serde_derive::{Deserialize, Serialize};

use actix_web::{web, Responder};

use foos;
use foos::database::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateSeriesRequest {
	players: [i32; 4],
}

pub fn create(
	request: web::Json<CreateSeriesRequest>,
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
	let response = foos::create_series(db.connection(), request.players, user_id);
	web::Json(response)
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateGauntletRequest {
	players: [i32; 5],
}

pub fn create_gauntlet(
	request: web::Json<CreateGauntletRequest>,
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
	let response = foos::create_gauntlet(db.connection(), request.players, user_id);
	web::Json(response)
}
