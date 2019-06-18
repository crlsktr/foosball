use crate::models::*;
use diesel::sqlite::SqliteConnection;
use diesel::RunQueryDsl;
use tera::Context;
use tera::Tera;

pub fn get_leaders(_request: &rouille::Request, connection: &SqliteConnection, templates: &Tera) -> rouille::Response {
    let leaders: Vec<Leader> = match diesel::sql_query(LEADER_BOARD_QUERY)
    .load(&*connection) {
        Ok(l) => l,
        Err(_) => return rouille::Response::text(format!("Well that sucks. {}", "Couldn't run the leaders query.")).with_status_code(500)
    };

    let mut context = Context::new();
    context.insert("leaders", &leaders);
    let html = match templates.render("leaders.html", &context) {
        Ok(t) => t,
        Err(_) => return rouille::Response::text(format!("Well that sucks. {}", "There is a problem with the leaders template")).with_status_code(500)
    };
    rouille::Response::html(html)
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
	,ifnull(min(case
		when games.wins = 1 then games.spread
		else null
		end), 0) as lowest_winning_spread
	,ifnull(min(case
		when games.wins <> 1 then games.spread
		else null
		end), 0) as lowest_losing_spread
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