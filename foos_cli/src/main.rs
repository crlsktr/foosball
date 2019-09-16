use clap::{App, Arg};
use foos::database::{ConnectionPool, Database};
use foos::run_pending_migrations;

mod config;
mod input;
mod player;
mod record;
mod series;
mod user;
mod replay;

use config::Config;

fn main() {
	// Setup the Command Line interface
	let version = "0.1.0";
	let author = "The foosball app team";

	let users_sub = user::get_user_command(author, version);
	let player_sub = player::get_player_command(author, version);
	let series_sub = series::get_series_command(author, version);
	let record_sub = record::get_record_command(author, version);
	let replay_sub = replay::get_replay_command(author, version);

	let matches = App::new("Foosball CLI")
		.version(version)
		.author(author)
		.about("Helps manage foosball tracking application and database")
		.arg(
			Arg::with_name("debug")
				.short("d")
				.long("debug")
				.value_name("Debug")
				.help("Adds debug output to console")
				.takes_value(false),
		)
		.arg(
			Arg::with_name("use-config")
				.short("c")
				.long("config")
				.value_name("Use Config")
				.help(
					"Use a config file instead of prompts for database-url, username, and password",
				)
				.takes_value(false),
		)
		.arg(
			Arg::with_name("database-url")
				.long("database-url")
				.value_name("Database URL")
				.required(false)
				.help("Sets the database url to use"),
		)
		.arg(
			Arg::with_name("username")
				.long("username")
				.value_name("Username")
				.required(false)
				.help("Sets the username to use"),
		)
		.arg(
			Arg::with_name("password")
				.long("password")
				.value_name("Password")
				.required(false)
				.help("Sets the password to use"),
		)
		.subcommands(vec![users_sub, player_sub, series_sub, record_sub, replay_sub])
		.get_matches();

	let debug = matches.is_present("debug");
	let use_config = matches.is_present("use-config");
	let mut ask_save_config = false;

	let mut config = if use_config {
		if let Some(config_path) = config::config_location() {
			config::FoosCliConfig::from_toml_file(config_path)
		} else {
			config::FoosCliConfig::from_defaults()
		}
	} else {
		config::FoosCliConfig::from_defaults()
	};

	// Database Url
	if config.database_url.is_none() {
		if let Some(db_url) = matches.value_of("database-url") {
			config.database_url = Some(db_url.to_string())
		} else {
			let mut url = String::new();
			while url.trim().is_empty() {
				url = input::get_input("Enter the Database URL: ");
			}
			config.database_url = Some(url.trim().to_string());
			ask_save_config = true;
		}
	}
	// Username
	if config.username.is_none() {
		if let Some(username) = matches.value_of("username") {
			config.username = Some(username.to_string())
		} else {
			let mut username = String::new();
			while username.trim().is_empty() {
				username = input::get_input("Username: ");
			}
			config.username = Some(username.trim().to_string());
			ask_save_config = true;
		}
	}
	// Pasword
	if config.password.is_none() {
		if let Some(password) = matches.value_of("password") {
			config.password = Some(password.to_string())
		} else {
			let mut password = String::new();
			while password.trim().is_empty() {
				password = input::get_input("password: ");
			}
			config.password = Some(password.trim().to_string());
			ask_save_config = true;
		}
	}

	if ask_save_config {
		let mut password = String::new();
		while password.trim().is_empty()
			|| (password.trim() != "y"
				&& password.trim() != "n"
				&& password.trim() != "yes"
				&& password.trim() != "no")
		{
			password = input::get_input("save config(y/n): ");
		}
		if password == "yes" || password == "y" {
			if let Some(path) = config::config_location() {
				match config.save(path) {
					Ok(_) => {}
					Err(_e) => {
						println!("Couldn't write config file...proceeding");
					}
				}
			} else {
				println!("Couldn't find the config directory...proceeding");
			}
		}
	}

	let database_url = config
		.database_url
		.expect("We should have a database url...");
	if debug {
		println!("Using conneciton to {}", &database_url);
	}

	// Get the database
	let db: Database = {
		let connection_pool: ConnectionPool = match ConnectionPool::create(database_url) {
			Ok(cp) => cp,
			Err(e) => {
				println!("No Connection Pool: {}", e);
				return;
			}
		};

		match connection_pool.get() {
			Ok(db) => db,
			Err(e) => {
				println!("No Connection: {}", e);
				return;
			}
		}
	};

	run_pending_migrations(db.connection());

	let username = config.username.expect("You must supply a username");
	let password = config.password.expect("You must supply a password");
	let user_id = match foos::user::authenticate(db.connection(), &username, &password) {
		Ok(u) => u.id,
		Err(_e) => {
			println!("Couldn't authenticate with supplied username and password");
			return;
		}
	};

	// Start select subcommand
	if let Some(matches) = matches.subcommand_matches("user") {
		user::entry(debug, &db, matches);
	}

	if let Some(matches) = matches.subcommand_matches("player") {
		player::entry(debug, &db, matches, user_id);
	}

	if let Some(matches) = matches.subcommand_matches("series") {
		series::entry(debug, &db, matches, user_id);
	}

	if let Some(matches) = matches.subcommand_matches("record") {
		record::entry(debug, &db, matches, user_id);
	}

	if let Some(_matches) = matches.subcommand_matches("replay") {
		replay::entry(&db);
	}
}
