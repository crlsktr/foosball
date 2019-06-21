
use diesel::prelude::*;
use models::*;

pub fn find_team(p1: &str, p2: &str, connection: &SqliteConnection) -> Option<Team> {
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

pub fn find_player(player_name: &str, connection: &SqliteConnection) -> Option<Player> {
	use schema::players::dsl::*;

	let player = players
		.filter(name.eq(player_name))
		.first::<Player>(connection);

	match player {
		Ok(p) => Some(p),
		Err(_) => None,
	}
}

pub fn game_win_team(game_id: i32) -> String {
	format!(r#"SELECT p.* FROM 'games' as g
				JOIN 'results' as r
				on r.game_id = g.id
				join 'teams' as t
				on t.id = r.winning_team
				join 'players' as p
				on p.id = t.player_one_id or p.id = t.player_two_id
				WHERE g.id = {};"#,
			game_id)
}

pub fn game_los_team(game_id: i32) -> String {
	format!(r#"
	select p.* from 'teams' as t
	join (
	select case WHEN r.winning_team = g.team_two_id then g.team_one_id else g.team_two_id end as los_team_id from 'games' as g
		join 'results' as r 
		on g.id = r.game_id
		where g.id = {}
	) as los_team_id
	on t.id = los_team_id
	join 'players' as p
	on p.id = t.player_one_id or p.id = t.player_two_id;"#, 
	game_id)
}