use diesel::prelude::*;
use diesel::sql_query;
use diesel::sql_types::{Integer, Varchar, Bool, Timestamptz};
use diesel::PgConnection;
use chrono::prelude::*;

#[derive(Serialize, Deserialize, QueryableByName)]
pub struct PlayerStats {
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
	#[sql_type = "Integer"]
	pub played: i32,
	#[sql_type = "Varchar"]
	pub percentage: String,
	#[sql_type = "Varchar"]
	pub best_team_mate: String,
	#[sql_type = "Integer"]
	pub best_team_ranking: i32,
	#[sql_type = "Varchar"]
	pub best_team_percentage: String,
	#[sql_type = "Varchar"]
	pub worst_team_mate: String,
	#[sql_type = "Integer"]
	pub worst_team_ranking: i32,
	#[sql_type = "Varchar"]
	pub worst_team_percentage: String,
}

pub fn player_stats(connection: &PgConnection, player_id: i32) -> Result<PlayerStats, String> {
	let player_stats = sql_query(PLAYER_STATS_QUERY)
		.bind::<Integer, _>(player_id)
		.get_result(connection)
		.map_err(|e| format!("Couldn't load the leaders. Error: {}", e))?;
	Ok(player_stats)
}

#[derive(Serialize, Deserialize, QueryableByName)]
pub struct PlayerGames {
	#[sql_type = "Varchar"]
	pub team_mate_name: String,
	#[sql_type = "Varchar"]
	pub opponent_one: String,
	#[sql_type = "Varchar"]
	pub opponent_two: String,
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
	game_ranking: i32,
}

pub fn player_games(connection: &PgConnection, player_id: i32) -> Result<Vec<PlayerGames>, String> {
	let games = sql_query(PLAYER_GAMES_QUERY)
		.bind::<Integer, _>(player_id)
		.load(connection)
		.map_err(|e| format!("Couldn't load the players games. Error: {}", e))?;
	Ok(games)
}

const PLAYER_GAMES_QUERY: &'static str = r#"
-- Player games query
SELECT
	team_mate.name as team_mate_name
	,o1.name as opponent_one
	,o2.name as opponent_two
	,CAST(CASE WHEN g.winners = t.id THEN 1 ELSE 0 END AS BOOLEAN) AS won
	,CAST(CASE WHEN g.winners = t.id THEN 10 ELSE 10 - g.spread END AS INT) AS points
	,CAST(CASE WHEN g.winners = t.id THEN 10 - g.spread ELSE 10 END AS INT) AS opponent_points
	,s.played_on
	,pr.change
	,pr.current_ranking as game_ranking
FROM players p
JOIN teams t
	ON t.player_one_id = p.id
	OR t.player_two_id = p.id
JOIN players team_mate
	ON (team_mate.id = t.player_one_id
		OR team_mate.id = t.player_two_id)
	AND team_mate.id != p.id
JOIN games g
	ON g.team_one_id = t.id
	OR g.team_two_id = t.id
JOIN player_rankings pr
	ON pr.player_id = p.id
	AND pr.game_id = g.id
JOIN player_rankings tmr
	ON tmr.player_id = team_mate.id
	AND tmr.game_id = g.id
JOIN teams against
	ON (against.id = g.team_one_id
	OR against.id = g.team_two_id)
	AND against.id != t.id
JOIN players o1
	ON against.player_one_id = o1.id
JOIN player_rankings o1r
	ON o1r.player_id = o1.id
	AND o1r.game_id = g.id
JOIN players o2
	ON against.player_two_id = o2.id
JOIN player_rankings o2r
	ON o2r.player_id = o2.id
	AND o2r.game_id = g.id
JOIN series s
	ON s.id = g.series_id
WHERE p.id = $1
AND g.winners IS NOT NULL
ORDER BY played_on DESC, pr.id DESC
"#;

const PLAYER_STATS_QUERY: &'static str = r#"
-- Player stats query
SELECT
	t.position
	,t.name
	,t.ranking
	,t.won
	,t.lost
	,t.played
	,t.percentage
	,t.best_team_mate
	,t.best_team_ranking
	,t.best_team_percentage
	,t.worst_team_mate
	,t.worst_team_ranking
	,t.worst_team_percentage
FROM 
(
	SELECT
		CAST(ROW_NUMBER() OVER (ORDER BY p.ranking DESC) AS INT) AS position
		,p.name
		,p.id as player_id
		,CAST(p.ranking AS INT) AS ranking
		,CAST(SUM(CASE WHEN t.id = g.winners THEN 1 ELSE 0 END) AS INT) AS won
		,CAST(SUM(CASE WHEN t.id = g.winners THEN 0 ELSE 1 END) AS INT) AS lost
		,CAST(COUNT(g.id) AS INT) AS played
		,CASE WHEN
			SUM(CASE WHEN t.id = g.winners THEN 0 ELSE 1 END) > 0
			THEN TO_CHAR(CAST(SUM(CASE WHEN t.id = g.winners THEN 1 ELSE 0 END) AS FLOAT) / CAST(COUNT(g.id) AS FLOAT), 'FM0.00')
		ELSE '1.00' END AS percentage
		,best_team.name as best_team_mate
		,best_team.ranking as best_team_ranking
		,best_team.percentage as best_team_percentage
		,worst_team.name as worst_team_mate
		,worst_team.ranking as worst_team_ranking
		,worst_team.percentage as worst_team_percentage
	FROM players p
	JOIN LATERAL (
		SELECT
			bt.id
			,bt.ranking
			,bp.name
			,CASE WHEN
				SUM(CASE WHEN bt.id = bg.winners THEN 0 ELSE 1 END) > 0
				THEN TO_CHAR(CAST(SUM(CASE WHEN bt.id = bg.winners THEN 1 ELSE 0 END) AS FLOAT) / CAST(COUNT(bg.id) AS FLOAT), 'FM0.00')
			ELSE '1.00' END AS percentage
		FROM teams bt
		JOIN players bp
			ON bt.player_one_id = bp.id
			OR bt.player_two_id = bp.id
		JOIN games bg
			ON bg.team_one_id = bt.id
			OR bg.team_two_id = bt.id
		WHERE bp.id != p.id
		AND (
			bt.player_one_id = p.id 
			OR bt.player_two_id = p.id
		)
		AND bg.winners IS NOT NULL
		GROUP BY bt.id, bt.ranking, bp.name
		ORDER BY bt.ranking desc
		LIMIT 1
	) best_team ON true
	JOIN LATERAL (
		SELECT
			bt.id
			,bt.ranking
			,bp.name
			,CASE WHEN
				SUM(CASE WHEN bt.id = bg.winners THEN 0 ELSE 1 END) > 0
				THEN TO_CHAR(CAST(SUM(CASE WHEN bt.id = bg.winners THEN 1 ELSE 0 END) AS FLOAT) / CAST(COUNT(bg.id) AS FLOAT), 'FM0.00')
			ELSE '1.00' END AS percentage
		FROM teams bt
		JOIN players bp
			ON bt.player_one_id = bp.id
			OR bt.player_two_id = bp.id
		JOIN games bg
			ON bg.team_one_id = bt.id
			OR bg.team_two_id = bt.id
		WHERE bp.id != p.id
		AND (
			bt.player_one_id = p.id 
			OR bt.player_two_id = p.id
		)
		AND bg.winners IS NOT NULL
		GROUP BY bt.id, bt.ranking, bp.name
		ORDER BY bt.ranking asc
		LIMIT 1
	) worst_team ON true
	JOIN teams t
		ON t.player_one_id = p.id
		OR t.player_two_id = p.id
	JOIN games g
		ON g.team_one_id = t.id
		OR g.team_two_id = t.id
	WHERE g.winners IS NOT NULL
	GROUP BY p.name, p.id, p.ranking, best_team.name, best_team.ranking, best_team.percentage, worst_team.name, worst_team.ranking, worst_team.percentage
	ORDER BY p.ranking DESC, won DESC, lost ASC, percentage DESC
) t
WHERE  t.player_id = $1
"#;
