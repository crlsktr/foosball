use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Read;
use std::path::Path;

mod foos_web_config;
pub use foos_web_config::*;

pub trait Config: Serialize + for<'de> Deserialize<'de> {
	fn from_toml_file<P: AsRef<Path>>(path: P) -> Self {
		let mut toml_string = String::new();
		let mut toml_file = match File::open(path.as_ref()) {
			Ok(f) => f,
			Err(_) => {
				println!(
					"could not open config file {:?} ... using defaults.",
					path.as_ref().to_str().unwrap_or("")
				);
				return Self::from_defaults();
			}
		};
		let _ = toml_file.read_to_string(&mut toml_string);
		let toml_string = toml_string.trim();

		if toml_string.len() == 0 {
			println!(
				"config file {:?} is empty ... using defaults.",
				path.as_ref().to_str().unwrap_or("")
			);
			return Self::from_defaults();
		}

		match toml::from_str(toml_string) {
			Ok(c) => c,
			Err(e) => {
				println!(
					"invalid config file {:?} ... using defaults",
					path.as_ref().to_str().unwrap_or("")
				);
				println!("the config error is: {:?}", e);
				Self::from_defaults()
			}
		}
	}

	fn save<P: AsRef<Path>>(&self, path: P) -> Result<(), std::io::Error> {
		use std::fs;
		use std::io::prelude::*;
		let toml = toml::to_string(&self).unwrap();
		let mut file = fs::File::create(path).expect("invalid path for config file.");
		file.write_all(toml.as_bytes())
	}

	fn from_defaults() -> Self;
}