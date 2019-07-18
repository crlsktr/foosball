use actix_web::{web, Responder};

use foos::database::*;
use foos;

pub fn get_leader_board(pool: web::Data<ConnectionPool>) -> impl Responder {
    let db = match pool.get() {
		Ok(db) => db,
		Err(e) => return web::Json(Err(format!("Couldn't get the database: {}", e))),
	};
    let response = foos::reports::leader_board(db.connection());
    web::Json(response)
}
