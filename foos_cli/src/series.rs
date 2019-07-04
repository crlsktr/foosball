use clap::{App, Arg, ArgMatches, SubCommand};
use foos::database::Database;

pub fn get_series_command<'a, 'b>(author: &'a str, version: &'a str) -> App<'a, 'b> {
	let add_start_sub = SubCommand::with_name("start")
		.about("starts a series of games")
		.version(version)
		.author(author)
		.arg(
			Arg::with_name("players")
				.required(true)
				.help("The player ids that will be playing the games")
				.min_values(4)
				.max_values(5),
		);

	let users_sub = SubCommand::with_name("series")
		.about("start/search series of games")
		.version(version)
		.author(author)
		.subcommands(vec![add_start_sub]); //search_sub

	users_sub
}

pub fn entry(debug: bool, db: &Database, matches: &ArgMatches) {
	if let Some(matches) = matches.subcommand_matches("start") {
		start(debug, db, matches);
	}
}

fn start(_debug: bool, db: &Database, matches: &ArgMatches) {
	let players: Vec<i32> = matches
		.values_of("players")
		.unwrap()
		.into_iter()
		.map(|v| v.parse::<i32>().unwrap_or(0))
		.collect();

	if players.iter().any(|p| *p == 0) {
		println!("Cannot use none integers as player id's");
		return;
	}

	match players.len() {
		4 => {
			let mut array = [0; 4];
			array.copy_from_slice(&players[..4]);
			start_series(_debug, db, array);
		}
		5 => {
			let mut array = [0; 5];
			array.copy_from_slice(&players[..5]);
			start_gauntlet(_debug, db, array);
		}
		_ => unreachable!(),
	}
}

fn start_series(_debug: bool, db: &Database, players: [i32; 4]) {
	let series = match foos::create_series(db.connection(), players) {
		Ok(s) => s,
		Err(e) => {
			println!("Couldn't Start Series: {}", e);
			return;
		}
	};
	show_series(series);
}

fn start_gauntlet(_debug: bool, db: &Database, players: [i32; 5]) {
	let series = match foos::create_gauntlet(db.connection(), players) {
		Ok(s) => s,
		Err(e) => {
			println!("Couldn't Start Gauntlet: {}", e);
			return;
		}
	};
	show_series(series);
}

fn show_series(series: foos::Series) {
	println!("");
	println!("Series {}", series.id);
	println!("{:-<104}", "");
	println!(
		"| {0: <7} | {1: <8} | {2: <30} | {3: <2} | {4: <8} | {5: <30} |",
		"Game Id", "Team One", "Players", "VS", "Team Two", "Players"
	);
	for game in series.games {
		let team_one_players = format!(
			"{0: >13} & {1: <13}",
			game.team_one.player_one.name, game.team_one.player_two.name
		);
		let team_two_players = format!(
			"{0: >13} & {1: <13}",
			game.team_two.player_one.name, game.team_two.player_two.name
		);
		println!(
			"| {0: <7} | {1: <8} | {2: <30} | {3: <2} | {4: <8} | {5: <30} |",
			game.id, game.team_one.id, team_one_players, "vs", game.team_two.id, team_two_players
		);
	}
	println!("{:-<1$}", "", 104);
	println!("");
}
