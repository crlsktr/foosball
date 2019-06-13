mod game;
mod player;
mod matches;
mod result;
mod team;

pub use self::game::*;
pub use self::player::*;
pub use self::matches::*;
pub use self::result::*;
pub use self::team::*;

 use diesel::sql_types::{Varchar, Integer, Float};

#[derive( Serialize)]
pub struct GameView {
    pub match_id: i32,
    pub game_id: i32,
    pub team_one: TeamView,
    pub team_two: TeamView, 
}

#[derive( Serialize)]
pub struct TeamView {
    pub team_id: i32,
    pub player_one: PlayerView,
    pub player_two: PlayerView,
}

#[derive( Serialize)]
pub struct PlayerView {
    pub player_id: i32,
    pub name: String,
}

#[derive(Serialize, QueryableByName )]
pub struct Leader {
    #[sql_type = "Varchar"]
    pub player_name: String,
    #[sql_type = "Integer"]
    pub games_won: i32,
    #[sql_type = "Integer"]
    pub games_lost: i32,
    #[sql_type = "Integer"]
    pub games_played: i32,
    #[sql_type = "Float"]
    pub percentage: f32,
    #[sql_type = "Integer"]
    pub highest_winning_spread: i32,
    #[sql_type = "Integer"]
    pub highest_losing_spread: i32,
    #[sql_type = "Integer"]
    pub lowest_winning_spread: i32,
    #[sql_type = "Integer"]
    pub lowest_losing_spread: i32,
    #[sql_type = "Integer"]
    pub average_winning_spread: i32,
    #[sql_type = "Integer"]
    pub average_losing_spread: i32,
}