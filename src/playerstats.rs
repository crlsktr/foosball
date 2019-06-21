use crate::models::*;
use diesel::sqlite::SqliteConnection;
use diesel::RunQueryDsl;
use tera::Context;
use tera::Tera;
use crate::shared_queries::*;


pub fn get_player_stats(request: &rouille::Request, connection: &SqliteConnection, templates: &Tera) -> rouille::Response {
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
    let stats: Vec<PlayerStats> =
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
    let html = match templates.render("player_stats.html", &context) {
        Ok(t) => t,
        Err(_) => return rouille::Response::text(format!("Well that sucks. {}", "There is a problem with the player stats template.")).with_status_code(500)
    };
    rouille::Response::html(html)
}


fn get_player_stats_query(player_id: i32) -> String {
    format!(
        r#"
        select
            p.ranking as rank
            ,sum(case when r.winning_team = t.id then 1 else 0 end) as won
            ,sum(case when r.winning_team <> t.id then 1 else 0 end) as lost
            ,count(g.id) as played
            ,cast(sum(case when r.winning_team = t.id then 1 else 0 end) as real) / cast(count(g.id) as real) as percentage
        from games g
        join results r
            on g.id = r.game_id
        join teams t
            on g.team_one_id = t.id 
            or g.team_two_id = t.id
        join players p
            on t.player_one_id = p.id
            or t.player_two_id = p.id
        where p.id = {}
        ;"#,
        player_id
    )
}
