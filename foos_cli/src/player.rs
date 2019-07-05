use clap::value_t;
use clap::{App, Arg, ArgMatches, SubCommand};
use foos::database::Database;

pub fn get_player_command<'a, 'b>(author: &'a str, version: &'a str) -> App<'a, 'b> {
	let add_player_sub = SubCommand::with_name("add")
		.about("adds a player")
		.version(version)
		.author(author)
		.arg(
			Arg::with_name("nickname")
				.required(true)
				.help("The nickname for the created player")
				.index(1),
		)
		.arg(
			Arg::with_name("userid")
				.required(false)
				.help("Link the player to a given user")
				.index(2),
		);

	let search_player_sub = SubCommand::with_name("search")
		.about("searches for existing players")
		.version(version)
		.author(author)
		.arg(
			Arg::with_name("searchterm")
				.required(false)
				.help("The string of text to use to search")
				.index(1),
		);

	let users_sub = SubCommand::with_name("player")
		.about("manages players")
		.version(version)
		.author(author)
		.subcommands(vec![add_player_sub, search_player_sub]);

	users_sub
}

pub fn entry(debug: bool, db: &Database, matches: &ArgMatches, user_id: i32) {
	if let Some(matches) = matches.subcommand_matches("add") {
		add_player(debug, db, matches, user_id);
	}

	if let Some(matches) = matches.subcommand_matches("search") {
		search_players(debug, db, matches);
	}
}

fn add_player(_debug: bool, db: &Database, matches: &ArgMatches, created_by: i32) {
	use foos::player::Player;
	let name = matches.value_of("nickname").unwrap();
	let user_id: Option<i32> = match value_t!(matches, "userid", i32) {
		Ok(u_id) => Some(u_id),
		Err(_) => None,
	};
	let player_result = Player::create(db.connection(), user_id, name, created_by);
	match player_result {
		Ok(p) => println!("Created player: {:?}", p),
		Err(e) => println!("{}", e),
	}
}

fn search_players(_debug: bool, db: &Database, matches: &ArgMatches) {
	use foos::player::Player;
	let term = matches.value_of("searchterm").unwrap_or("");

	let search_result = Player::search(db.connection(), term, 10);

	match search_result {
		Ok(players) => {
			println!("");
			println!(
				"| {0: <10} | {1: <10} | {2: <6} | {3: <50} |",
				"Player Id", "User Id", "Rank", "Name"
			);
			for player in players {
				let user_id = match player.user_id {
					Some(id) => format!("{}", id),
					None => "".to_string(),
				};
				println!(
					"| {0: <10} | {1: <10} | {2: <6} | {3: <50} |",
					player.id, user_id, player.ranking, player.name
				);
			}
			println!("");
		}
		Err(e) => println!("{}", e),
	}
}
