use crate::schema::teams;
use diesel::prelude::*;
use diesel::PgConnection;

#[derive(Queryable, QueryableByName, Serialize, Debug, Clone)]
#[table_name = "teams"]
pub struct Team {
	pub id: i32,
	pub player_one_id: i32,
	pub player_two_id: i32,
	pub ranking: i32,
}

#[derive(Insertable, Deserialize)]
#[table_name = "teams"]
pub struct NewTeam {
	pub player_one_id: i32,
	pub player_two_id: i32,
	pub ranking: i32,
}

impl Team {
	pub fn create(
		connection: &PgConnection,
		player_one_id: i32,
		player_two_id: i32,
	) -> Result<Team, String> {
		if let Ok(team) = Team::find(connection, player_one_id, player_two_id) {
			return Ok(team);
		};
		let new_team = NewTeam {
			player_one_id,
			player_two_id,
			ranking: 1500,
		};
		let team = diesel::insert_into(teams::table)
			.values(&new_team)
			.get_result(connection)
			.map_err(|e| format!("Couldn't create team: {}", e))?;

		Ok(team)
	}

	pub fn find(
		connection: &PgConnection,
		player_one_id: i32,
		player_two_id: i32,
	) -> Result<Team, String> {
		use teams::dsl as t;
		let team: Team = t::teams
			.filter(
				t::player_one_id
					.eq(player_one_id)
					.and(t::player_two_id.eq(player_two_id))
					.or(t::player_one_id
						.eq(player_two_id)
						.and(t::player_two_id.eq(player_one_id))),
			)
			.first::<Team>(connection)
			.map_err(|e| format!("Unable to find for team: {}", e))?;
		Ok(team)
	}
}
