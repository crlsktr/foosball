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
