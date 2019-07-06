use crate::config::Config;
use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FoosWebConfig {
	pub database_url: String,
	pub bind_url: String,
	pub secure_cookies: bool,
}

impl Config for FoosWebConfig {
	fn from_defaults() -> FoosWebConfig {
		FoosWebConfig {
			database_url: "".to_string(),
			bind_url: "127.0.0.1:8000".to_string(),
			secure_cookies: true,
		}
	}
}
