use chrono::prelude::*;
use diesel::prelude::*;
use diesel::sql_query;
use diesel::sql_types::{Integer, Varchar, Bool, Timestamptz};
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

#[derive(Serialize, Deserialize, QueryableByName)]
pub struct TeamGames {
	#[sql_type = "Varchar"]
	pub opposing_player_one: String,
	#[sql_type = "Varchar"]
	pub opposing_player_two: String,
	#[sql_type = "Integer"]
	pub opposing_team_id: i32,
	#[sql_type = "Bool"]
	pub won: bool,
	#[sql_type = "Integer"]
	pub points: i32,
	#[sql_type = "Integer"]
	pub opponent_points: i32,
	#[sql_type = "Timestamptz"]
	pub played_on: DateTime<Utc>,
	#[sql_type = "Integer"]
	change: i32,
	#[sql_type = "Integer"]
	current_ranking: i32,
}

pub fn team_games(
	connection: &PgConnection,
	team_id : i32
) -> Result <Vec<TeamGames>, String> {
	let team_games = sql_query (TEAM_GAMES_QUERY)
		.bind::<Integer, _>(team_id)
		.load(connection)
		.map_err(|err| format!("Couldn't load the games. Error {}", err))?;

	Ok(team_games)
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

const TEAM_GAMES_QUERY: &'static str = r#"
SELECT 
	tlp1.name as opposing_player_one, 
	tlp2.name as opposing_player_two,
	o_team.id as opposing_team_id,
	CAST(CASE WHEN g.winners = t.id THEN 1 ELSE 0 END AS BOOLEAN) AS won,
	CAST(CASE WHEN g.winners = t.id THEN 10 ELSE 10 - g.spread END AS INT) AS points,
	CAST(CASE WHEN g.winners = t.id THEN 10 - g.spread ELSE 10 END AS INT) AS opponent_points,
	s.played_on,
	tr.change,
	tr.current_ranking
FROM 
teams t
JOIN games g
ON
	g.team_one_id = t.id OR 
	g.team_two_id = t.id
JOIN teams o_team
ON
	o_team.id <> t.id AND (g.team_one_id = o_team.id OR g.team_two_id = o_team.id)
JOIN players tlp1 --player 1 team left
ON
	tlp1.id = o_team.player_one_id
JOIN players tlp2 --player 2 team left
ON
	tlp2.id = o_team.player_two_id
JOIN series s
ON 
	g.series_id = s.id
JOIN team_rankings tr
ON
	tr.team_id = t.id AND
	tr.game_id = g.id
WHERE t.id = 11
ORDER BY s.played_on DESC;
"#;

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
