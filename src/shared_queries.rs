
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