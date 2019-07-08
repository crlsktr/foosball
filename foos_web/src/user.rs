use serde_derive::{Deserialize, Serialize};
use actix_session::{Session};

use actix_web::{web, Responder};

use foos::database::*;

pub fn search(
	path: web::Path<String>,
	pool: web::Data<ConnectionPool>,
) -> impl Responder {
	let db = match pool.get() {
		Ok(db) => db,
		Err(e) => return web::Json(Err(format!("Couldn't get the database: {}", e))),
	};
	let term = path.into_inner();
	let term = if term == "all" { "".to_string() } else { term };
	let response = foos::user::search(db.connection(), &term);
	web::Json(response)
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateUserRequest {
	username: String,
	password: String,
}

pub fn create(
	request: web::Json<CreateUserRequest>,
	pool: web::Data<ConnectionPool>,
) -> impl Responder {
	let db = match pool.get() {
		Ok(db) => db,
		Err(e) =>  return web::Json(Err(format!("Couldn't get the database: {}", e))),
	};
	let request = request.into_inner();
	let response = foos::user::create_user(db.connection(), &request.username, &request.password);
	web::Json(response)
}


#[derive(Serialize, Deserialize, Debug)]
pub struct AuthenticateRequest {
	username: String,
	password: String,
}

pub fn authenticate(
	request: web::Json<AuthenticateRequest>,
	pool: web::Data<ConnectionPool>,
	session: Session,
) -> impl Responder {
	let db = match pool.get() {
		Ok(db) => db,
		Err(e) =>  return web::Json(Err(format!("Couldn't get the database: {}", e))),
	};
	let request = request.into_inner();
	let response = foos::user::authenticate(db.connection(), &request.username, &request.password);
	// TODO: need to add a cookie here.
	match &response {
		Ok(u) => {
			let _ = session.set("user_id", u.id);
		},
		Err(_) => {}
	}
	web::Json(response)
}

