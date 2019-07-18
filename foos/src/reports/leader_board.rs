use diesel::prelude::*;
use diesel::PgConnection;
use diesel::sql_types::{Varchar, Integer};

#[derive(Serialize, Deserialize, QueryableByName)]
pub struct Leader {
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

pub fn leader_board(connection: &PgConnection) -> Result<Vec<Leader>, String> {
    let leaders: Vec<Leader> = diesel::sql_query(LEADER_BOARD_QUERY)
        .load(connection)
        .map_err(|e| format!("Couldn't load the leaders. Error: {}", e))?;
    Ok(leaders)
}

const LEADER_BOARD_QUERY: &'static str = r#"
SELECT
	pg.name
	,CAST(pg.ranking AS INT) as ranking
	,CAST(SUM(pg.won) AS INT) as won
	,CAST(SUM(CASE WHEN pg.won = 0 THEN 1 ELSE 0 END) AS INT) as lost
	,CAST(COUNT(pg.game_id) as INT) as played
	,TO_CHAR(CAST(SUM(pg.won) AS FLOAT) / CAST(SUM(CASE WHEN pg.won = 0 THEN 1 ELSE 0 END) AS FLOAT), 'FM0.00') as percentage
FROM 
(
	SELECT
		p.id 
		,p.name
		,p.ranking
		,g.id as game_id
		,CASE
			WHEN g.winners = g.team_one_id AND (p.id = g.player_one_id OR p.id = g.player_two_id) THEN 1
			WHEN g.winners = g.team_two_id AND (p.id = g.player_three_id OR p.id = g.player_four_id) THEN 1
		ELSE 0 END AS won
		,g.spread
	FROM players p
	JOIN (
		SELECT
			g.id
			,t1.team_id as team_one_id
			,t2.team_id as team_two_id
			,winners
			,spread
			,t1.p1_id as player_one_id
			,t1.p2_id as player_two_id
			,t2.p1_id as player_three_id
			,t2.p2_id as player_four_id
		FROM games g
		JOIN (
			SELECT
				t.id as team_id
				,p1.id as p1_id
				,p2.id as p2_id
			FROM teams t
			JOIN players p1 ON t.player_one_id = p1.id
			JOIN players p2 on t.player_two_id = p2.id
		) t1 ON t1.team_id = g.team_one_id
		JOIN (
			SELECT
				t.id as team_id
				,p1.id as p1_id
				,p2.id as p2_id
			FROM teams t
			JOIN players p1 ON t.player_one_id = p1.id
			JOIN players p2 on t.player_two_id = p2.id
		) t2 ON t2.team_id = g.team_two_id
	) g
	ON g.player_one_id = p.id
	OR g.player_two_id = p.id
	OR g.player_three_id = p.id
	OR g.player_four_id = p.id
	--WHERE g.winners IS NOT NULL
	ORDER BY p.id DESC
) pg
GROUP BY pg.id, pg.name, pg.ranking
ORDER BY pg.ranking desc, won desc, lost asc, percentage desc
"#;
