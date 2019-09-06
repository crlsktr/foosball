use crate::config::Config;
use serde_derive::{Deserialize, Serialize};
use std::env;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FoosWebConfig {
	pub database_url: String,
	pub bind_url: String,
	pub secure_cookies: bool,
}

impl Config for FoosWebConfig {
	fn from_defaults() -> FoosWebConfig {
		let database_url = match env::var("DB_HOST") {
			Ok(dbh) => dbh,
			Err(_) => "".to_string(),
		};

		let bind_url = match env::var("BIND_URL") {
			Ok(burl) => burl,
			Err(_) => "127.0.0.1:8000".to_string(),
		};

		FoosWebConfig {
			database_url,
			bind_url,
			secure_cookies: true,
		}
	}
}
