use diesel::prelude::*;
use diesel::sql_query;
use diesel::sql_types::{Integer, Varchar};
use diesel::PgConnection;

#[derive(Serialize, Deserialize, QueryableByName)]
pub struct TeamStats {
	#[sql_type = "Integer"]
	pub position: i32,
	#[sql_type = "Varchar"]
	pub player_one_name: String,
	#[sql_type = "Varchar"]
	pub player_two_name: String,
	#[sql_type = "Integer"]
	pub won: i32,
	#[sql_type = "Integer"]
	pub lost: i32,
	#[sql_type = "Integer"]
	pub played: i32,
	#[sql_type = "Varchar"]
	pub percentage: String, // yes this is a string....
}

pub fn team_stats(
	connection: &PgConnection,
	player_one_id: i32,
	player_two_id: i32,
) -> Result<TeamStats, String> {
	let player_stats = sql_query(TEAM_STATS_QUERY)
		.bind::<Integer, _>(player_one_id)
		.bind::<Integer, _>(player_two_id)
		.get_result(connection)
		.map_err(|e| format!("Couldn't load the leaders. Error: {}", e))?;
	Ok(player_stats)
}

const TEAM_STATS_QUERY: &'static str = r#"
-- Player stats query
SELECT
	ts.position
	,ts.player_one_name
	,ts.player_two_name
	,ts.won
	,ts.lost
	,ts.played
	,ts.percentage
FROM (
	SELECT
		CAST(ROW_NUMBER() OVER (ORDER BY tg.won DESC, tg.lost ASC, tg.percentage DESC) AS INT) AS position
		,tg.player_one_id
		,tg.player_two_id
		,tg.player_one_name
		,tg.player_two_name
		,tg.won
		,tg.lost
		,tg.played
		,tg.percentage
	FROM (
		SELECT
			p1.id AS player_one_id
			,p2.id AS player_two_id
			,p1.name AS player_one_name
			,p2.name AS player_two_name
			,CAST(SUM(CASE WHEN t.id = g.winners THEN 1 ELSE 0 END) AS INT) AS won
			,CAST(SUM(CASE WHEN t.id = g.winners THEN 0 ELSE 1 END) AS INT) AS lost
			,CAST(COUNT(g.id) AS INT) AS played
			,CASE WHEN
				SUM(CASE WHEN t.id = g.winners THEN 0 ELSE 1 END) > 0
				THEN TO_CHAR(CAST(SUM(CASE WHEN t.id = g.winners THEN 1 ELSE 0 END) AS FLOAT) / CAST(COUNT(g.id) AS FLOAT), 'FM0.00')
			ELSE '1.00' END AS percentage
		FROM teams t
		JOIN players p1
			ON t.player_one_id = p1.id
		JOIN players p2
			ON t.player_two_id = p2.id
		JOIN games g
			ON g.team_one_id = t.id OR g.team_two_id = t.id
		GROUP BY t.id, p1.id, p2.id, p1.name, p2.name
	) tg
	ORDER BY tg.won DESC, tg.lost ASC, tg.percentage DESC
) ts
WHERE 
	(ts.player_one_id = $1 and ts.player_two_id = $2) 
	OR (ts.player_two_id = $1 and ts.player_one_id = $2)
"#;
