#![allow(proc_macro_derive_resolution_fallback)]

#[macro_use]
extern crate rouille;
#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate diesel;
extern crate inflector;
#[macro_use]
extern crate tera;
extern crate chrono;

use chrono::prelude::*;
use inflector::Inflector;
use tera::Context;
use tera::Tera;

use self::diesel::prelude::*;
//use diesel::sqlite;
use diesel::sqlite::SqliteConnection;
//use diesel::sql_types::Date;
use std::sync::Mutex;

pub mod models;
pub mod schema;

use models::*;

fn main() {
	let db = {
		let db = SqliteConnection::establish("./foosball.db");
		Mutex::new(db.expect("Failed to connect to ./foosball.db"))
	};

	let templates: Tera = compile_templates!("templates/**/*");

	rouille::start_server("0.0.0.0:12346", move |request| {
		let connection = db.lock().expect("database in use");
		router!(request,
			(POST) (/results) => {
				let r = try_or_400!(post_input!(request, {

					game_0: i32,
					spread_0: i32,
					winner_0: i32,

					game_1: i32,
					spread_1: i32,
					winner_1: i32,

					game_2: i32,
					spread_2: i32,
					winner_2: i32,

					game_3: Option<i32>,
					spread_3: Option<i32>,
					winner_3: Option<i32>,

					game_4: Option<i32>,
					spread_4: Option<i32>,
					winner_4: Option<i32>,
				}));

				create_results(r.game_0, r.spread_0, r.winner_0, &*connection);
				create_results(r.game_1, r.spread_1, r.winner_1, &*connection);
				create_results(r.game_2, r.spread_2, r.winner_2, &*connection);
				if r.game_3.is_some() && r.spread_3.is_some() && r.winner_3.is_some() {
					create_results(r.game_3.unwrap(), r.spread_3.unwrap(), r.winner_3.unwrap(), &*connection);
				}
				if r.game_4.is_some() && r.spread_4.is_some() && r.winner_4.is_some() {
					create_results(r.game_4.unwrap(), r.spread_4.unwrap(), r.winner_4.unwrap(), &*connection);
				}
				rouille::Response::html("Results Saved")
			},

			(POST) (/play) => {
				let players = try_or_400!(post_input!(request, {
					player_0: String,
					player_1: String,
					player_2: String,
					player_3: String,
					player_4: Option<String>,
				}));

				let mut player_selection = vec![players.player_0, players.player_1, players.player_2, players.player_3 ];
				if players.player_4.is_some() {
					player_selection.push(players.player_4.unwrap());
				}
				let players = player_selection;

				let match_ = NewMatch {
					is_gauntlet: if players.len() > 4 { true } else { false },
					date: Utc::now().to_rfc3339(),
				};

				diesel::insert_into(schema::matches::table)
					.values(match_)
					.execute(&*connection)
					.expect("couldn't start match");

				// load match we just inserted
				let new_match = schema::matches::dsl::matches
					.order_by(schema::matches::id.desc())
					.first::<Match>(&*connection)
					.expect("couldn't load match");

				let (games, teams, players) = match new_match.is_gauntlet {
					true => create_gauntlet(new_match.id, &players, &*connection),
					false => create_match(new_match.id, &players, &*connection),
				};
				let mut game_views = vec!();
				for game in games {
					let view = GameView {
						match_id: new_match.id,
						game_id: game.id,
						team_one: {
								let team = teams.iter().find(|t| t.id == game.team_one_id).unwrap();
								let player_one_id = team.player_one_id;
								let player_two_id =team.player_two_id;
								let p1_name = &players.iter().find(|p| p.id == player_one_id).unwrap().name;
								let p2_name = &players.iter().find(|p| p.id == player_two_id).unwrap().name;

								TeamView {
								team_id: game.team_one_id,
								player_one: PlayerView {
									player_id: player_one_id,
									name: p1_name.clone(),
								},
								player_two: PlayerView {
									player_id: player_two_id,
									name: p2_name.clone(),
								}
							}
						},
						team_two: {
								let team = teams.iter().find(|t| t.id == game.team_two_id).unwrap();
								let player_one_id = team.player_one_id;
								let player_two_id =team.player_two_id;
								let p1_name = &players.iter().find(|p| p.id == player_one_id).unwrap().name;
								let p2_name = &players.iter().find(|p| p.id == player_two_id).unwrap().name;

								TeamView {
								team_id: game.team_two_id,
								player_one: PlayerView {
									player_id: player_one_id,
									name: p1_name.clone(),
								},
								player_two: PlayerView {
									player_id: player_two_id,
									name: p2_name.clone(),
								}
							}
						}
					};
					game_views.push(view);
				}

				let mut context = Context::new();
				context.insert("is_gauntlet", &new_match.is_gauntlet);
				context.insert("match_number", &new_match.id);
				context.insert("games", &game_views);
				let html = templates.render("play.html", &context).unwrap();
				rouille::Response::html(html.clone())
			},

			(GET) (/) => {
                use schema::players::dsl::*;
                let plyrs: Vec<Player> = players.filter(id.gt(0))
					.load::<Player>(&*connection)
					.expect("Error Querying players");

				let mut context = Context::new();
                context.insert("players", &plyrs);
				let html = templates.render("index.html", &context).unwrap();
				rouille::Response::html(html.clone())
			},

			(GET) (/new/{match_type: String}) => {
				use schema::players::dsl::*;

				let mut context = Context::new();

				let plyrs: Vec<Player> = players.filter(id.gt(0))
					.load::<Player>(&*connection)
					.expect("Error Querying players");
				let num_players = match match_type.as_str() {
					"gauntlet" => 5,
					_ => 4,
				};
				let num_players: Vec<u32> = (0..num_players).map(|i| i).collect();

				context.insert("match_type", &match_type.to_title_case());
				context.insert("players", &plyrs);
				context.insert("num_players", &num_players);
				let html = templates.render("new_match.html", &context).unwrap();
				rouille::Response::html(html)
			},

			(GET) (/leaderboard) => {
				let leaders: Vec<Leader> = diesel::sql_query(LEADER_BOARD_QUERY)
				.load(&*connection).unwrap();

				let mut context = Context::new();
				context.insert("leaders", &leaders);
				let html = templates.render("leaders.html", &context).unwrap();
				rouille::Response::html(html)
			},

            (GET) (/playerstats) => {
                let name = request.get_param("player_name");

                if name.is_none() {
                    return rouille::Response::empty_400();
                }
                let name= name.unwrap();

                let player = match find_player(&name, &*connection){
                    Some(p) => p,
                    _ => return rouille::Response::html(format!("No Stats For {}", name))
                };

                let query = get_player_stats_query(player.id);
				let stats: Vec<TeamStats> =
					diesel::sql_query(query)
					.load(&*connection)
					.unwrap_or(vec!());

				let stats = match stats.get(0) {
                    Some(s) => s,
                    _ => return rouille::Response::html(format!("No Stats For {}", name))
                };

                let mut context = Context::new();
                context.insert("name", &name);
                context.insert("stats", &stats);
				let html = templates.render("player_stats.html", &context).unwrap();
				rouille::Response::html(html)
            },

			(GET) (/teamstats) => {
                ///{player_one: String}/{player_two: String}
                let player_one = request.get_param("player_one");
                let player_two = request.get_param("player_two");

                if player_one.is_none() || player_two.is_none() {
                    return rouille::Response::empty_400();
                }

                let player_one = player_one.unwrap();
                let player_two = player_two.unwrap();

				let team = find_team(&player_one, &player_two, &*connection);
				let team = match team {
					Some(t) => t,
					_ =>  return rouille::Response::html(format!("Couldn't find {} and {} ever being on a team.", player_one, player_two))
				};

				let query = get_team_stats_query(team.id);
				let stats: Vec<TeamStats> =
					diesel::sql_query(query)
					.load(&*connection)
					.unwrap();

				let stats = match stats.get(0) {
                    Some(s) => s,
                    _ => return rouille::Response::html(format!("Couldn't find {} and {} ever being on a team.", player_one, player_two))
                };

                let query = get_team_stats_against_query(team.id);
				let vs_stats: Vec<VsStats> =
					diesel::sql_query(query)
					.load(&*connection)
					.unwrap();

				let mut context = Context::new();
				context.insert("stats", &stats);
                context.insert("vs_stats", &vs_stats);
				context.insert("player_one", &player_one);
				context.insert("player_two", &player_two);
				let html = templates.render("team_stats.html", &context).unwrap();
				rouille::Response::html(html)
			},

			(GET) (/api/{id: u32}) => {
				println!("u32 {:?}", id);

				rouille::Response::empty_400()
			},

			(GET) (/api/{id: String}) => {
				println!("String {:?}", id);
				rouille::Response::text(format!("hello, {}", id))
			},

			_ => rouille::Response::empty_404()
		)
	});
}

fn create_gauntlet(
	match_id: i32,
	player_names: &Vec<String>,
	connection: &SqliteConnection,
) -> (Vec<Game>, Vec<Team>, Vec<Player>) {
	let players = get_players(player_names, connection);
    
	if players.len() != 5 {
		println!("Not enough players for a gauntlet");
	}
	let mut ordered_players = vec![];

	for name in player_names {
		let player = players.iter().find( |p| &p.name == name ).unwrap();
		ordered_players.push(player.clone());
	}



	let mut teams: Vec<Team> = vec![];
	let team_selection = vec![
		(0, 1), // 0 - p1,p2
		(0, 2), // 1 - p1,p3
		(0, 3), // 2 - p1,p4
		(0, 4), // 3 - p1,p5
		(1, 2), // 4 - p2,p3
		(1, 3), // 5 - p2,p4
		(1, 4), // 6 - p2,p5
		(2, 3), // 7 - p3,p4
		(2, 4), // 8 - p3,p5
		(3, 4), // 9 - p4,p5
	];

	for team in team_selection {
		let p1 = ordered_players.get(team.0).unwrap();
		let p2 = ordered_players.get(team.1).unwrap();
		let team = create_team(p1, p2, connection);
		teams.push(team);
	}

	let mut games: Vec<Game> = vec![];
	let game_selection = vec![
        (4, 9), // p2,p3 vs p4,p5
        (7, 3), // p3,p4 vs p1,p5
        (2, 6), // p1,p4 vs p2,p5
        (0, 8), // p1,p2 vs p3,p3
        (5, 1)  // p2,p4 vs p1,p3
    ];
	for game in game_selection {
		let t1 = teams.get(game.0).unwrap();
		let t2 = teams.get(game.1).unwrap();
		let game = create_game(t1, t2, match_id, connection);
		games.push(game)
	}

	(games, teams, players)
}

fn create_match(
	match_id: i32,
	player_names: &Vec<String>,
	connection: &SqliteConnection,
) -> (Vec<Game>, Vec<Team>, Vec<Player>) {
	let players = get_players(player_names, connection);
	if players.len() != 4 {
		println!("Not enough players for a match");
	}

	let mut teams: Vec<Team> = vec![];
	let team_selection = vec![(0, 1), (2, 3), (1, 2), (0, 3), (1, 3), (0, 2)];

	for team in team_selection {
		let p1 = players.get(team.0).unwrap();
		let p2 = players.get(team.1).unwrap();
		let team = create_team(p1, p2, connection);
		teams.push(team);
	}

	let mut games: Vec<Game> = vec![];
	let game_selection = vec![(0, 1), (2, 3), (4, 5)];
	for game in game_selection {
		let t1 = teams.get(game.0).unwrap();
		let t2 = teams.get(game.1).unwrap();
		let game = create_game(t1, t2, match_id, connection);
		games.push(game)
	}

	(games, teams, players)
}

fn create_game(t1: &Team, t2: &Team, match_id_: i32, connection: &SqliteConnection) -> Game {
	use schema::games::dsl::*;
	let new_game = NewGame {
		match_id: match_id_,
		team_one_id: t1.id,
		team_two_id: t2.id,
	};

	diesel::insert_into(schema::games::table)
		.values(new_game)
		.execute(connection)
		.expect("couldn't create new game");

	games
		.order_by(id.desc())
		.first::<Game>(connection)
		.expect("couldn't load new game")
}

fn create_team(p1: &Player, p2: &Player, connection: &SqliteConnection) -> Team {
	use schema::teams::dsl::*;

	let existing_team = teams
		.filter(player_one_id.eq_any(&[p1.id, p2.id]))
		.filter(player_two_id.eq_any(&[p1.id, p2.id]))
		.first::<Team>(connection);

	match existing_team {
		Ok(t) => t,
		Err(_) => {
			let new_team = NewTeam {
				player_one_id: p1.id,
				player_two_id: p2.id,
			};

			diesel::insert_into(schema::teams::table)
				.values(new_team)
				.execute(connection)
				.expect("couldn't create new team");

			teams
				.filter(player_one_id.eq_any(&[p1.id, p2.id]))
				.filter(player_two_id.eq_any(&[p1.id, p2.id]))
				.first::<Team>(connection)
				.expect("couldn't load created team")
		}
	}
}

fn find_team(p1: &str, p2: &str, connection: &SqliteConnection) -> Option<Team> {
	use schema::teams::dsl::*;

	let p1 = find_player(p1, connection);
	let p2 = find_player(p2, connection);
	if p1.is_none() || p2.is_none() {
		return None;
	}

	let p1 = p1.unwrap();
	let p2 = p2.unwrap();

	let team = teams
		.filter(player_one_id.eq_any(&[p1.id, p2.id]))
		.filter(player_two_id.eq_any(&[p1.id, p2.id]))
		.first::<Team>(connection);

	match team {
		Ok(t) => Some(t),
		Err(_) => None,
	}
}

fn find_player(player_name: &str, connection: &SqliteConnection) -> Option<Player> {
	use schema::players::dsl::*;

	let player = players
		.filter(name.eq(player_name))
		.first::<Player>(connection);

	match player {
		Ok(p) => Some(p),
		Err(_) => None,
	}
}

fn get_players(player_names: &Vec<String>, connection: &SqliteConnection) -> Vec<Player> {
	use schema::players::dsl::*;
	let mut existing_players = players
		.filter(name.eq_any(player_names))
		.load::<Player>(connection)
		.expect("Couldn't load players");

	// If we didn't find all the players should we create them.
	if existing_players.len() == player_names.len() {
		existing_players
	} else {
		for player in player_names {
			if !existing_players.iter().any(|p| &p.name == player) {
				let new_player = NewPlayer {
					name: player.to_string(),
				};
				diesel::insert_into(schema::players::table)
					.values(new_player)
					.execute(connection)
					.expect("couldn't create new team");

				let new_player = players
					.filter(name.eq(player))
					.first::<Player>(connection)
					.expect("couldn't load created team");
				existing_players.push(new_player);
			}
		}
		existing_players
	}
}

fn create_results(game_id: i32, spread: i32, winning_team_id: i32, connection: &SqliteConnection) {
	let game_result = NewResult {
		game_id,
		winning_team: winning_team_id,
		spread,
	};

	diesel::insert_into(schema::results::table)
		.values(game_result)
		.execute(connection)
		.expect("couldn't create game results");
}

const LEADER_BOARD_QUERY: &'static str = r#"
select
	p.name as player_name
	,ifnull(sum(games.wins), 0) as games_won
	,ifnull(count(games.wins) - sum(games.wins), 0) as games_lost
	,ifnull(count(games.wins), 0) as games_played
	,case 
		when count(games.wins) <> 0 then
			ifnull(cast(ifnull(sum(games.wins), 0) as real) / cast(count(games.wins) as real), 1)
		else 0
	end as percentage
	,max(case
		when games.wins = 1 then games.spread
		else 0
		end) as highest_winning_spread
	,max(case
		when games.wins <> 1 then games.spread
		else 0
		end) as highest_losing_spread
	,min(case
		when games.wins = 1 then games.spread
		else 0
		end) as lowest_winning_spread
	,min(case
		when games.wins <> 1 then games.spread
		else 0
		end) as lowest_losing_spread
	,ifnull(sum(case
		when games.wins = 1 then ifnull(games.spread, 0)
		else 0
		end) / sum(
		case
		when games.wins = 1 then 1
		else 0
		end), 0) as average_winning_spread
	,ifnull(sum(case
		when games.wins <> 1 then ifnull(games.spread, 0)
		else 0
		end) / sum(
		case
		when games.wins <> 1 then 1
		else 0
		end), 0) as average_losing_spread
from players p
left join (
	select
		-- *
		g.id
		,t.id
		,r.spread
		,t.player_one_id
		,t.player_two_id
		,case
			when r.winning_team = t.id then 1
			else 0
		end as wins
	from games g
	join results r
		on r.game_id = g.id
	join teams t
		on (g.team_one_id = t.id
		or g.team_two_id = t.id)
	) games
	on games.player_one_id = p.id or games.player_two_id = p.id
group by p.id
order by 
	percentage desc
	,average_winning_spread desc
	,highest_winning_spread desc
	,average_losing_spread asc
	,highest_losing_spread asc
	,lowest_winning_spread asc
	,lowest_losing_spread desc
	,games_played desc
;"#;

fn get_team_stats_query(team_id: i32) -> String {
	format!(
		r#"
    select
        sum(case 
            when r.winning_team = t.id then 1
            else 0 end) as won
        ,sum(case
            when r.winning_team <> t.id then 1
            else 0 end) as lost
        ,count(g.id) as played
		,cast(sum(case 
            when r.winning_team = t.id then 1
            else 0 end) as real) / cast(count(g.id) as real) as percentage
    from teams t
    join games g
        on (g.team_one_id = t.id or g.team_two_id = t.id)
    join results r
        on r.game_id = g.id
    where t.id = {}
    group by t.id
    ;"#,
		team_id
	)
}

fn get_team_stats_against_query(team_id: i32) -> String {
    	format!(
		r#"
    select
		p3.name as player_three
		,p4.name as player_four
		,sum(case when r.winning_team = t1.id then 1 else 0 end) as won
		,sum(case when r.winning_team <> t1.id then 1 else 0 end) as lost
		,count(g.id) as played
		,cast(sum(case 
            when r.winning_team = t1.id then 1
            else 0 end) as real) / cast(count(g.id) as real) as percentage
    from teams t1
    join games g
        on (g.team_one_id = t1.id or g.team_two_id = t1.id)
	join teams t2
		on (g.team_one_id = t2.id and g.team_one_id <> t1.id)
		or (g.team_two_id = t2.id and g.team_two_id <> t1.id)
	join players p3
		on t2.player_one_id = p3.id
	join players p4
		on t2.player_two_id = p4.id
    join results r
        on r.game_id = g.id
    where t1.id = {}
	group by t2.id
    ;"#,
		team_id
	)
}

fn get_player_stats_query(player_id: i32) -> String {
    format!(
        r#"
        select
            sum(case when r.winning_team = t.id then 1 else 0 end) as won
            ,sum(case when r.winning_team <> t.id then 1 else 0 end) as lost
            ,count(g.id) as played
            ,cast(sum(case when r.winning_team = t.id then 1 else 0 end) as real) / cast(count(g.id) as real) as percentage
        from games g
        join results r
            on g.id = r.game_id
        join teams t
            on g.team_one_id = t.id 
            or g.team_two_id = t.id
        where t.player_one_id = {}
            or t.player_two_id = {}
        ;"#,
        player_id,
        player_id
    )
}
