use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Read;
use std::path::Path;
use dirs;
use std::path::PathBuf;

mod foos_cli_config;
pub use foos_cli_config::*;

pub fn config_location() -> Option<PathBuf> {
    let config_dir = dirs::config_dir();
    if let Some(mut config_dir) = config_dir {
        config_dir.push("foos_cli.config");
        return Some(config_dir);
    }
    None
}

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

#[cfg(test)]
mod tests {
	use std::fs;
	use std::io::prelude::*;
	use Config;

	const TEST_PATH: &'static str = "test_config.toml";

	#[derive(Serialize, Deserialize, Debug)]
	struct TestConfig {
		value: String,
		maybe_some: Option<String>,
		maybe_none: Option<String>,
	}

	impl Config for TestConfig {
		fn from_defaults() -> TestConfig {
			TestConfig {
				value: "Default Test Config".to_owned(),
				maybe_some: Some("Default Test Config".to_owned()),
				maybe_none: None,
			}
		}
	}

	fn make_test_file() {
		let test_contents = r#"value = "test"
maybe_some = "test"
"#;
		let mut test_file = fs::File::create(TEST_PATH).unwrap();
		let _ = test_file.write_all(test_contents.as_bytes()).unwrap();
	}

	#[test]
	fn default_config() {
		let test_config = TestConfig::from_defaults();
		assert!(&test_config.value == "Default Test Config");
		assert!(test_config.maybe_some == Some("Default Test Config".to_owned()));
		assert!(test_config.maybe_none.is_none());
	}

	#[test]
	fn load_config() {
		make_test_file();
		let test_config = TestConfig::from_toml_file(TEST_PATH);
		assert!(&test_config.value == "test");
		assert!(test_config.maybe_some == Some("test".to_owned()));
		assert!(test_config.maybe_none.is_none());
		let _ = fs::remove_file(TEST_PATH);
	}
}
