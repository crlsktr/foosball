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

use inflector::Inflector;
use tera::Tera;
use tera::Context;
use chrono::prelude::*;

use self::diesel::prelude::*;
//use diesel::sqlite;
use diesel::sqlite::SqliteConnection;
//use diesel::sql_types::Date;
use std::sync::{Mutex};

pub mod schema;
pub mod models;

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
            // (POST) (/api/savegames) => {
            //     let games: Vec<NewGame> = try_or_400!(rouille::input::json_input(request));
            //     for game in games.iter(){
            //         diesel::insert_into(schema::games::table)
            //             .values(game)
            //             .execute(&*connection)
            //             .expect("couldn't save games");
            //     } 

            //     rouille::Response::text("Ok")
            // },

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
                let context = Context::new();
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

fn create_gauntlet(match_id: i32, player_names: &Vec<String>, connection: &SqliteConnection) -> (Vec<Game>, Vec<Team>, Vec<Player>) {
    let players = get_players(player_names, connection);
    if players.len() != 5 {
        println!("Not enough players for a gauntlet");
    }

    let mut teams: Vec<Team> = vec!();
    let team_selection = vec!(
        (0, 1), (0, 2), (0, 3), (0, 4),
        (1, 2), (1, 3), (1, 4),
        (2, 3), (2, 4),
        (3, 4)
        );

    for team in team_selection {
        let p1 = players.get(team.0).unwrap();
        let p2 = players.get(team.1).unwrap();
        let team = create_team(p1, p2, connection);
        teams.push(team);
    }

    let mut games: Vec<Game> = vec!();
    let game_selection = vec!((4, 9), (7, 3), (2, 6), (0, 8), (5, 1));
    for game in game_selection {
        let t1 = teams.get(game.0).unwrap();
        let t2 = teams.get(game.1).unwrap();
        let game = create_game(t1, t2, match_id, connection);
        games.push(game)
    }

    (games, teams, players)
}



fn create_match(match_id: i32, player_names: &Vec<String>, connection: &SqliteConnection) -> (Vec<Game>, Vec<Team>, Vec<Player>) {
    let players = get_players(player_names, connection);
    if players.len() != 4 {
        println!("Not enough players for a match");
    }

    let mut teams: Vec<Team> = vec!();
    let team_selection = vec!(
        (0, 1), (2, 3), (1, 2), (0, 3), (1, 3), (0, 2)
        );

    for team in team_selection {
        let p1 = players.get(team.0).unwrap();
        let p2 = players.get(team.1).unwrap();
        let team = create_team(p1, p2, connection);
        teams.push(team);
    }

    let mut games: Vec<Game> = vec!();
    let game_selection = vec!((0, 1), (3, 4), (5, 6));
    for game in game_selection {
        let t1 = teams.get(game.0).unwrap();
        let t2 = teams.get(game.1).unwrap();
        let game = create_game(t1, t2, match_id, connection);
        games.push(game)
    }

    (games, teams, players)
}

fn create_game(t1: &Team, t2: &Team, match_id_ : i32, connection: &SqliteConnection) -> Game {
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

            teams.filter(player_one_id.eq_any(&[p1.id, p2.id]))
                .filter(player_two_id.eq_any(&[p1.id, p2.id]))
                .first::<Team>(connection)
                .expect("couldn't load created team")
        }
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
                    name: player.to_string()
                };
                diesel::insert_into(schema::players::table)
                    .values(new_player)
                    .execute(connection)
                    .expect("couldn't create new team");

                let new_player = players.filter(name.eq(player))
                    .first::<Player>(connection)
                    .expect("couldn't load created team");
                existing_players.push(new_player);
            }
        }
        existing_players
    }
}