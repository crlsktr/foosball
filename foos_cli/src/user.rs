use clap::{App, Arg, ArgMatches, SubCommand};
use foos::database::Database;

pub fn get_user_command<'a, 'b>(author: &'a str, version: &'a str) -> App<'a, 'b> {
	let add_user_sub = SubCommand::with_name("add")
		.about("adds a user")
		.version(version)
		.author(author)
		.arg(
			Arg::with_name("username")
				.required(true)
				.help("The username for the created user")
				.index(1),
		)
		.arg(
			Arg::with_name("password")
				.required(true)
				.help("The password for the created user")
				.index(2),
		);

	let search_user_sub = SubCommand::with_name("search")
		.about("search for existing users")
		.version(version)
		.author(author)
		.arg(
			Arg::with_name("searchterm")
				.required(false)
				.help("The string of text to use to search")
				.index(1),
		);

	let users_sub = SubCommand::with_name("user")
		.about("manages users")
		.version(version)
		.author(author)
		.subcommands(vec![add_user_sub, search_user_sub]);

	users_sub
}

pub fn entry(debug: bool, db: &Database, matches: &ArgMatches) {
	if let Some(matches) = matches.subcommand_matches("add") {
		add_user(debug, db, matches);
	}

	if let Some(matches) = matches.subcommand_matches("search") {
		search_user(debug, db, matches);
	}
}

fn add_user(_debug: bool, db: &Database, matches: &ArgMatches) {
	let username = matches.value_of("username").unwrap();
	let password = matches.value_of("password").unwrap();
	let user_result = foos::user::create_user(db.connection(), username, password);
	match user_result {
		Ok(u) => println!("Created user: {:?}", u),
		Err(e) => println!("{}", e),
	}
}

fn search_user(_debug: bool, db: &Database, matches: &ArgMatches) {
	let term = matches.value_of("searchterm").unwrap_or("");

	let search_result = foos::user::search(db.connection(), term);

	match search_result {
		Ok(users) => {
			println!("");
			println!("| {0: <10} | {1: <50}", "User Id", "Username");
			for user in users {
				println!("| {0: <10} | {1: <50}", user.id, user.username);
			}
			println!("");
		}
		Err(e) => println!("{}", e),
	}
}
