use crate::config::Config;
use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FoosCliConfig {
	pub database_url: Option<String>,
	pub username: Option<String>,
	pub password: Option<String>,
}

impl Config for FoosCliConfig {
	fn from_defaults() -> FoosCliConfig {
		FoosCliConfig {
			database_url: None,
			username: None,
			password: None,
		}
	}
}
