use crate::models::*;
use diesel::sqlite::SqliteConnection;
use diesel::RunQueryDsl;
use tera::Context;
use tera::Tera;
use crate::shared_queries::*;


pub fn get_team_stats(request: &rouille::Request, connection: &SqliteConnection, templates: &Tera) -> rouille::Response {
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
        .unwrap_or(vec!());

    let stats = match stats.get(0) {
        Some(s) => s,
        _ => return rouille::Response::html(format!("Couldn't find {} and {} ever being on a team.", player_one, player_two))
    };

    let query = get_team_stats_against_query(team.id);
    let vs_stats: Vec<VsStats> =
        diesel::sql_query(query)
        .load(&*connection)
        .unwrap_or(vec!());

    let mut context = Context::new();
    context.insert("stats", &stats);
    context.insert("vs_stats", &vs_stats);
    context.insert("player_one", &player_one);
    context.insert("player_two", &player_two);
    let html = match templates.render("team_stats.html", &context) {
        Ok(t) => t,
        Err(_) => return rouille::Response::text(format!("Well that sucks. {}", "There is a problem with the team stats template.")).with_status_code(500)
    };
    rouille::Response::html(html)
}


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
