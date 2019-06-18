use crate::models::*;
use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use diesel::RunQueryDsl;
use tera::Context;
use tera::Tera;
use inflector::Inflector;

pub fn new_match(_request: &rouille::Request, connection: &SqliteConnection, match_type: String, templates: &Tera) -> rouille::Response {
    use schema::players::dsl::*;

    let mut context = Context::new();

    let plyrs: Vec<Player> = match players.filter(id.gt(0))
        .load::<Player>(&*connection)
    {
        Ok(p) => p,
        Err(_) => return rouille::Response::text(format!("Well that sucks. {}", "Couldn't query the players table.")).with_status_code(500)
    };

    let num_players = match match_type.as_str() {
        "gauntlet" => 5,
        _ => 4,
    };
    let num_players: Vec<u32> = (0..num_players).map(|i| i).collect();

    context.insert("match_type", &match_type.to_title_case());
    context.insert("players", &plyrs);
    context.insert("num_players", &num_players);
    let html = match templates.render("new_match.html", &context) {
        Ok(t) => t,
        Err(_) => return rouille::Response::text(format!("Well that sucks. {}", "There is a problem with the new match template.")).with_status_code(500)
    };
    rouille::Response::html(html)
}