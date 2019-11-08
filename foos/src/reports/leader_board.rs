use diesel::prelude::*;
use diesel::sql_types::{Integer, Varchar};
use diesel::PgConnection;

#[derive(Serialize, Deserialize, QueryableByName)]
pub struct Leader {
	#[sql_type = "Integer"]
	pub player_id: i32,
	#[sql_type = "Integer"]
	pub position: i32,
	#[sql_type = "Varchar"]
	pub name: String,
	#[sql_type = "Integer"]
	pub ranking: i32,
	#[sql_type = "Integer"]
	pub won: i32,
	#[sql_type = "Integer"]
	pub lost: i32,
	#[sql_type = "Varchar"]
	pub respected: String,
	#[sql_type = "Integer"]
	pub played: i32,
	#[sql_type = "Varchar"]
	pub percentage: String, // yes this is a string....
}

pub fn leader_board(connection: &PgConnection) -> Result<Vec<Leader>, String> {
	let leaders: Vec<Leader> = diesel::sql_query(LEADER_BOARD_QUERY)
		.load(connection)
		.map_err(|e| format!("Couldn't load the leaders. Error: {}", e))?;
	Ok(leaders)
}

const LEADER_BOARD_QUERY: &'static str = r#"
-- leader board
SELECT
	p.id AS player_id
	,CAST(ROW_NUMBER() OVER (ORDER BY p.ranking DESC) AS INT) AS position
	,p.name
	,CAST(p.ranking AS INT) AS ranking
	,CAST(SUM(CASE WHEN t.id = g.winners THEN 1 ELSE 0 END) AS INT) AS won
	,CAST(SUM(CASE WHEN t.id = g.winners THEN 0 ELSE 1 END) AS INT) AS lost
	,CASE
		WHEN SUM(CASE WHEN t.id != g.winners THEN 1 ELSE 0 END) > 0 THEN
		TO_CHAR(CAST(SUM(CASE WHEN t.id <> g.winners AND g.spread <= 5 THEN 1 ELSE 0 END) AS FLOAT) / 
		CAST(SUM(CASE WHEN t.id != g.winners THEN 1 ELSE 0 END) AS FLOAT),  'FM0.00')
	ELSE '1.00' END AS respected
	,CAST(COUNT(g.id) AS INT) AS played
	,CASE WHEN
		SUM(CASE WHEN t.id = g.winners THEN 0 ELSE 1 END) > 0
		THEN TO_CHAR(CAST(SUM(CASE WHEN t.id = g.winners THEN 1 ELSE 0 END) AS FLOAT) / CAST(COUNT(g.id) AS FLOAT), 'FM0.00')
	ELSE '1.00' END AS percentage
FROM players p
JOIN teams t
	ON t.player_one_id = p.id
	OR t.player_two_id = p.id
JOIN games g
	ON g.team_one_id = t.id
	OR g.team_two_id = t.id
WHERE g.winners IS NOT NULL
GROUP BY p.name, p.id, p.ranking
ORDER BY p.ranking DESC, won DESC, lost ASC, percentage DESC
"#;

#[derive(Serialize, Deserialize, QueryableByName)]
pub struct TeamLeader {
	#[sql_type = "Integer"]
	pub team_id: i32,
	#[sql_type = "Integer"]
	pub position: i32,
	#[sql_type = "Varchar"]
	pub player_one_name: String,
	#[sql_type = "Varchar"]
	pub player_two_name: String,
	#[sql_type = "Integer"]
	pub ranking: i32,
	#[sql_type = "Integer"]
	pub won: i32,
	#[sql_type = "Integer"]
	pub lost: i32,
	#[sql_type = "Integer"]
	pub played: i32,
	#[sql_type = "Varchar"]
	pub percentage: String, // yes this is a string....
}

pub fn team_leader_board(connection: &PgConnection) -> Result<Vec<TeamLeader>, String> {
	let leaders: Vec<TeamLeader> = diesel::sql_query(TEAM_LEADER_BOARD_QUERY)
		.load(connection)
		.map_err(|e| format!("Couldn't load the team leaders. Error: {}", e))?;
	Ok(leaders)
}

const TEAM_LEADER_BOARD_QUERY: &'static str = r#"
SELECT
	t.id as team_id,
	CAST(ROW_NUMBER() OVER (ORDER BY t.ranking DESC) AS INT) AS position
	,p1.name AS player_one_name
	,p2.name AS player_two_name
	,CAST(t.ranking AS INT) AS ranking
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
GROUP BY t.id, p1.name, p2.name
ORDER BY t.ranking DESC
"#;
