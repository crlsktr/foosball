use serde_derive::{Deserialize, Serialize};

use actix_web::{ web, Responder};

use foos::database::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct UserSearchRequest {
    term: String
}

pub fn search_user(
    search_request: web::Json<UserSearchRequest>,
    pool: web::Data<ConnectionPool>,
) -> impl Responder {
    let db = match pool.get() {
        Ok(db) => db,
        Err(e) => return web::Json(Err(format!("Couldn't get the database: {}", e))),
    };
    let response = foos::user::search(db.connection(), &search_request.into_inner().term);
    return web::Json(response)
}