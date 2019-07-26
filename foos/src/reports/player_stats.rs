use diesel::prelude::*;
use diesel::PgConnection;
use diesel::sql_types::{Varchar, Integer, BigInt, Array};
use diesel::sql_query;
use diesel::dsl::sql;

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
    pub percentage: String, // yes this is a string.... 
}

pub fn player_stats(connection: &PgConnection, player_id: i32) -> Result<PlayerStats, String> {
    let player_stats = sql_query(PLAYER_STATS_QUERY)
        .bind::<Integer, _>(player_id)
        .get_result(connection)
        .map_err(|e| format!("Couldn't load the leaders. Error: {}", e))?;
    Ok(player_stats)
}

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
			THEN TO_CHAR(CAST(SUM(CASE WHEN t.id = g.winners THEN 1 ELSE 0 END) AS FLOAT) / CAST(SUM(CASE WHEN t.id = g.winners THEN 0 ELSE 1 END) AS FLOAT), 'FM0.00')
		ELSE '1.00' END AS percentage
	FROM players p
	JOIN teams t
		ON t.player_one_id = p.id
		OR t.player_two_id = p.id
	JOIN games g
		ON g.team_one_id = t.id
		OR g.team_two_id = t.id
	GROUP BY p.name, p.id, p.ranking
	ORDER BY p.ranking DESC, won DESC, lost ASC, percentage DESC
) t
WHERE  t.player_id = $1
"#;
