use crate::schema;
use crate::models::*;
use crate::rank::probability_win;
use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use diesel::RunQueryDsl;
use chrono::prelude::*;
use tera::Context;
use tera::Tera;

pub fn play_match(request: &rouille::Request, connection: &SqliteConnection, templates: &Tera) -> rouille::Response {
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

    match diesel::insert_into(schema::matches::table)
        .values(match_)
        .execute(&*connection)
	{
		Ok(_) => {},
		Err(e) => return rouille::Response::text(format!("Well that sucks. {}. Error: {}", "Couldn't insert new matches", e)).with_status_code(500)
	};

    // load match we just inserted
    let new_match = match schema::matches::dsl::matches
        .order_by(schema::matches::id.desc())
        .first::<Match>(&*connection)
	{
		Ok(m) => m,
		Err(_) => return rouille::Response::text(format!("Well that sucks. {}", "Couldn't load new match.")).with_status_code(500)
	};

    let (games, teams, players) = match new_match.is_gauntlet {
        true => match create_gauntlet(new_match.id, &players, &*connection) {
			Ok(g) => g,
			Err(_) => return rouille::Response::text(format!("Well that sucks. {}", "Couldn't create the guantlet")).with_status_code(500)
		},
        false => match create_match(new_match.id, &players, &*connection){
			Ok(m) => m,
			Err(_) => return rouille::Response::text(format!("Well that sucks. {}", "Couldn't create the match")).with_status_code(500)
		},
    };
    let mut game_views = vec!();
    for game in games {
		let team_one = teams.iter().find(|t| t.id == game.team_one_id).unwrap();
        let team_one_player_one_id = team_one.player_one_id;
        let team_one_player_two_id = team_one.player_two_id;

		let team_two = teams.iter().find(|t| t.id == game.team_two_id).unwrap();
		let team_two_player_one_id = team_two.player_one_id;
		let team_two_player_two_id = team_two.player_two_id;

		let team_one_ranking = 
			&players.iter().find(|p| p.id == team_one_player_one_id).unwrap().ranking + 
			&players.iter().find(|p| p.id == team_one_player_two_id).unwrap().ranking;

		let team_two_ranking = 
			&players.iter().find(|p| p.id == team_two_player_one_id).unwrap().ranking + 
			&players.iter().find(|p| p.id == team_two_player_two_id).unwrap().ranking;


        let view = GameView {
            match_id: new_match.id,
            game_id: game.id,
            team_one: {
                    let p1_name = &players.iter().find(|p| p.id == team_one_player_one_id).unwrap().name;
                    let p2_name = &players.iter().find(|p| p.id == team_one_player_two_id).unwrap().name;

                    TeamView {
                    team_id: game.team_one_id,
                    player_one: PlayerView {
                        player_id: team_one_player_one_id,
                        name: p1_name.clone(),
                    },
                    player_two: PlayerView {
                        player_id: team_one_player_two_id,
                        name: p2_name.clone(),
                    }
                }
            },
            team_two: {
                    
                    let p1_name = &players.iter().find(|p| p.id == team_two_player_one_id).unwrap().name;
                    let p2_name = &players.iter().find(|p| p.id == team_two_player_two_id).unwrap().name;

                    TeamView {
                    team_id: game.team_two_id,
                    player_one: PlayerView {
                        player_id: team_two_player_one_id,
                        name: p1_name.clone(),
                    },
                    player_two: PlayerView {
                        player_id: team_two_player_two_id,
                        name: p2_name.clone(),
                    }
                }
            },
			team_one_win_probability: format!("{:.2}",(probability_win(team_one_ranking as f32, team_two_ranking as f32) * 100.0)),
			team_two_win_probability: format!("{:.2}",(probability_win(team_two_ranking as f32, team_one_ranking as f32) * 100.0))
        };
        game_views.push(view);
    }

    let mut context = Context::new();
    context.insert("is_gauntlet", &new_match.is_gauntlet);
    context.insert("match_number", &new_match.id);
    context.insert("games", &game_views);
    let html = match templates.render("play.html", &context) {
		Ok(t) => t,
		Err(_) => return rouille::Response::text(format!("Well that sucks. {}", "There is a problem with the play template.")).with_status_code(500)
	};
    rouille::Response::html(html.clone())
}


fn create_gauntlet(
	match_id: i32,
	player_names: &Vec<String>,
	connection: &SqliteConnection,
) -> std::result::Result<(Vec<Game>, Vec<Team>, Vec<Player>), String> {
	let players = get_players(player_names, connection).map_err(|e| e)?;
    
	if players.len() != 5 {
		return Err("Not enough players for a gauntlet".to_string());
	}
	let mut ordered_players = vec![];

	for name in player_names {
		// This should always work.
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
		// These should always unwrap().
		let p1 = ordered_players.get(team.0).unwrap();
		let p2 = ordered_players.get(team.1).unwrap();
		let team = create_team(p1, p2, connection).map_err(|e| e)?;
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
		// these should always unwrap().
		let t1 = teams.get(game.0).unwrap();
		let t2 = teams.get(game.1).unwrap();
		let game = create_game(t1, t2, match_id, connection).map_err(|e| e)?;
		games.push(game)
	}

	Ok((games, teams, players))
}

fn create_match(
	match_id: i32,
	player_names: &Vec<String>,
	connection: &SqliteConnection,
) -> std::result::Result<(Vec<Game>, Vec<Team>, Vec<Player>), String> {
	let players = get_players(player_names, connection).map_err(|e| e)?;
	if players.len() != 4 {
		return Err("Not enough players for a match".to_string());
	}

	let mut teams: Vec<Team> = vec![];
	let team_selection = vec![(0, 1), (2, 3), (1, 2), (0, 3), (1, 3), (0, 2)];

	for team in team_selection {
		// leaving these unwraps they should never fail.
		let p1 = players.get(team.0).unwrap();
		let p2 = players.get(team.1).unwrap();
		let team = create_team(p1, p2, connection).map_err(|e| e)?;
		teams.push(team);
	}

	let mut games: Vec<Game> = vec![];
	let game_selection = vec![(0, 1), (2, 3), (4, 5)];
	for game in game_selection {
		let t1 = teams.get(game.0).unwrap();
		let t2 = teams.get(game.1).unwrap();
		let game = create_game(t1, t2, match_id, connection).map_err(|e| e)?;
		games.push(game)
	}

	Ok((games, teams, players))
}

fn get_players(player_names: &Vec<String>, connection: &SqliteConnection) -> std::result::Result<Vec<Player>, String> {
	use schema::players::dsl::*;
	let mut existing_players = players
		.filter(name.eq_any(player_names))
		.load::<Player>(connection)
		.map_err(|_| "Couldn't check for existing players".to_string())?;

	// If we didn't find all the players should we create them.
	if existing_players.len() == player_names.len() {
		Ok(existing_players)
	} else {
		for player in player_names {
			if !existing_players.iter().any(|p| &p.name == player) {
				let new_player = NewPlayer {
					name: player.to_string(),
				};
				diesel::insert_into(schema::players::table)
					.values(new_player)
					.execute(connection)
					.map_err(|_| "Couldn't create player".to_string())?;

				let new_player = players
					.filter(name.eq(player))
					.first::<Player>(connection)
					.map_err(|_| "Couldn't load new player".to_string())?;
				existing_players.push(new_player);
			}
		}
		Ok(existing_players)
	}
}

fn create_game(t1: &Team, t2: &Team, match_id_: i32, connection: &SqliteConnection) -> std::result::Result<Game, String> {
	use schema::games::dsl::*;
	let new_game = NewGame {
		match_id: match_id_,
		team_one_id: t1.id,
		team_two_id: t2.id,
	};

	diesel::insert_into(schema::games::table)
		.values(new_game)
		.execute(connection)
		.map_err(|_| "Couldn't create new game".to_string())?;

	let game = games
		.order_by(id.desc())
		.first::<Game>(connection)
		.map_err(|_| "Couldn't load new game".to_string())?;

	Ok(game)
}

fn create_team(p1: &Player, p2: &Player, connection: &SqliteConnection) -> std::result::Result<Team, String> {
	use schema::teams::dsl::*;

	let existing_team = teams
		.filter(player_one_id.eq_any(&[p1.id, p2.id]))
		.filter(player_two_id.eq_any(&[p1.id, p2.id]))
		.first::<Team>(connection);

	let team = match existing_team {
		Ok(t) => t,
		Err(_) => {
			let new_team = NewTeam {
				player_one_id: p1.id,
				player_two_id: p2.id,
			};

			diesel::insert_into(schema::teams::table)
				.values(new_team)
				.execute(connection)
				.map_err(|_| "Couldn't create new team".to_string())?;

			teams
				.filter(player_one_id.eq_any(&[p1.id, p2.id]))
				.filter(player_two_id.eq_any(&[p1.id, p2.id]))
				.first::<Team>(connection)
				.map_err(|_| "Couldn't load created team".to_string())?
		}
	};

	Ok(team)
}