use crate::models::*;
use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use diesel::RunQueryDsl;
use tera::Context;
use tera::Tera;


pub fn get_index(_request: &rouille::Request, connection: &SqliteConnection, templates: &Tera) -> rouille::Response {
    use schema::players::dsl::*;
    let plyrs: Vec<Player> = match players.filter(id.gt(0))
        .load::<Player>(&*connection)
    {
        Ok(p) => p,
         Err(_) => return rouille::Response::text(format!("Well that sucks. {}", "Couldn't query the players table.")).with_status_code(500)
    };

    let mut context = Context::new();
    context.insert("players", &plyrs);
    let html = match templates.render("index.html", &context) {
        Ok(t) => t,
        Err(_) => return rouille::Response::text(format!("Well that sucks. {}", "There is a problem with the index template")).with_status_code(500)
    };
    rouille::Response::html(html.clone())
}