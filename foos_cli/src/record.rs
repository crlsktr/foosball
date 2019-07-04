use clap::value_t;
use clap::{App, Arg, ArgMatches, SubCommand};
use foos::database::Database;
use foos::GameResult;

pub fn get_record_command<'a, 'b>(author: &'a str, version: &'a str) -> App<'a, 'b> {
	let game_sub = SubCommand::with_name("game")
		.about("records a single game ")
		.version(version)
		.author(author)
		.arg(
			Arg::with_name("game-id")
				.short("g")
				.long("game-id")
				.required(true)
				.help("The game id of the finish game that is being recorded")
				.takes_value(true),
		)
		.arg(
			Arg::with_name("winning-team-id")
				.short("w")
				.long("winning-team-id")
				.required(true)
				.help("The team id of the winning team")
				.takes_value(true),
		)
		.arg(
			Arg::with_name("spread")
				.short("s")
				.long("spread")
				.required(true)
				.help("The difference of the score")
				.takes_value(true),
		);

	// let games_sub = SubCommand::with_name("games")
	// 	.about("records multiple games")
	// 	.version(version)
	// 	.author(author)
	// 	.arg(
	// 		Arg::with_name("searchterm")
	// 			.required(false)
	// 			.help("The string of text to use to search")
	// 			.index(1),
	// 	);

	let record_sub = SubCommand::with_name("record")
		.about("record game results")
		.version(version)
		.author(author)
		.subcommands(vec![game_sub]);

	record_sub
}

pub fn entry(debug: bool, db: &Database, matches: &ArgMatches) {
	if let Some(matches) = matches.subcommand_matches("game") {
		record_game(debug, db, matches);
	}

	// if let Some(matches) = matches.subcommand_matches("games") {
	// 	search_user(debug, db, matches);
	// }
}

fn record_game(_debug: bool, db: &Database, matches: &ArgMatches) {
	let id = match value_t!(matches, "game-id", i32) {
		Ok(g) => g,
		Err(_) => {
			println!("game-id must be and integer");
			return;
		}
	};

	let winners = match value_t!(matches, "winning-team-id", i32) {
		Ok(w) => w,
		Err(_) => {
			println!("winning-team must be and integer");
			return;
		}
	};

	let spread = match value_t!(matches, "spread", i16) {
		Ok(s) => s,
		Err(_) => {
			println!("spread must be and integer");
			return;
		}
	};

	let game_result = GameResult {
		id,
		winners,
		spread,
	};

	let result = foos::finish_games(db.connection(), &vec![game_result]);
	match result {
		Ok(_) => {}
		Err(e) => println!("Couldn't record game result: {}", e),
	};
}
