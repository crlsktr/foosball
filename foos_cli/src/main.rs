use clap::{App, Arg};
use foos::database::{ConnectionPool, Database};

mod player;
mod record;
mod series;
mod user;

fn main() {
	// Setup the Command Line interface
	let version = "0.1.0";
	let author = "The foosball app team";

	let users_sub = user::get_user_command(author, version);
	let player_sub = player::get_player_command(author, version);
	let series_sub = series::get_series_command(author, version);
	let record_sub = record::get_record_command(author, version);

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
			Arg::with_name("database-url")
				.long("database-url")
				.value_name("Database URL")
				.required(true)
				.help("Sets the database url to use"),
		)
		.subcommands(vec![users_sub, player_sub, series_sub, record_sub])
		.get_matches();

	let debug = matches.is_present("debug");
	let database_url = matches.value_of("database-url").unwrap();

	if debug {
		println!("Using conneciton to {}", database_url);
	}

	// Get the database
	let db: Database = {
		let connection_pool: ConnectionPool = match ConnectionPool::create(database_url) {
			Ok(cp) => cp,
			Err(e) => {
				println!("No Connetion Pool: {}", e);
				return;
			}
		};

		match connection_pool.get() {
			Ok(db) => db,
			Err(e) => {
				println!("No Connetion: {}", e);
				return;
			}
		}
	};

	// Start select subcommand
	if let Some(matches) = matches.subcommand_matches("user") {
		user::entry(debug, &db, matches);
	}

	if let Some(matches) = matches.subcommand_matches("player") {
		player::entry(debug, &db, matches);
	}

	if let Some(matches) = matches.subcommand_matches("series") {
		series::entry(debug, &db, matches);
	}

	if let Some(matches) = matches.subcommand_matches("record") {
		record::entry(debug, &db, matches);
	}
}
