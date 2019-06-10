 #![allow(proc_macro_derive_resolution_fallback)]

#[macro_use]
extern crate rouille;
#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate diesel;

use self::diesel::prelude::*;
use diesel::sqlite;
use diesel::sqlite::SqliteConnection;
use diesel::sql_types::Date;
use std::sync::{Mutex};

pub mod schema;
pub mod models;

use models::Game;


fn main() {
    let db = {
        let db = SqliteConnection::establish("./foosball.db");
        Mutex::new(db.expect("Failed to connect to ./foosball.db"))
    };

    rouille::start_server("0.0.0.0:12346", move |request| {
        
        let connection = db.lock().expect("database in use");
        let response = rouille::match_assets(&request, ".");
        if response.is_success() {
            return response;
        }
        router!(request,
            (POST) (/api/savegames) => {
                let games: Vec<Game> = try_or_400!(rouille::input::json_input(request));
                for game in games.iter(){
                    diesel::insert_into(schema::games::table)
                        .values(game)
                        .execute(&*connection)
                        .expect("couldn't save games");
                } 

                rouille::Response::text("Ok")
            },

            (GET) (/api/{id: u32}) => {
                println!("u32 {:?}", id);

                rouille::Response::empty_400()
            },

            (GET) (/api/{id: String}) => {
                println!("String {:?}", id);
                rouille::Response::text(format!("hello, {}", id))
            },

            _ => rouille::Response::empty_404()
        )
    });
}