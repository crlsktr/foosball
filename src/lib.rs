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
extern crate tera;
extern crate chrono;

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
pub mod rank;