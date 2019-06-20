mod game;
mod matches;
mod player;
mod result;
mod team;

pub use self::game::*;
pub use self::matches::*;
pub use self::player::*;
pub use self::result::*;
pub use self::team::*;

use std::cmp::Ordering;
use diesel::sql_types::{Float, Integer, Varchar};

#[derive(Serialize)]
pub struct GameView {
	pub match_id: i32,
	pub game_id: i32,
	pub team_one: TeamView,
	pub team_two: TeamView,
}

#[derive(Serialize)]
pub struct TeamView {
	pub team_id: i32,
	pub player_one: PlayerView,
	pub player_two: PlayerView,
}

#[derive(Serialize)]
pub struct PlayerView {
	pub player_id: i32,
	pub name: String,
}

#[derive(Serialize, QueryableByName, Debug)]
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

impl PartialOrd for Leader {
	fn partial_cmp(&self, other: &Leader)-> Option<Ordering> {
		Some(self.cmp(other))
	}
}

impl Ord for Leader {
	fn cmp(&self, other: &Leader) -> Ordering {
		if self.percentage > other.percentage{
			Ordering::Less
		}
		else if self.percentage < other.percentage{
			Ordering::Greater
		}
		else if self.average_winning_spread > other.average_winning_spread{
			 Ordering::Less
		}
		else if self.average_winning_spread < other.average_winning_spread{
			 Ordering::Greater
		}
		else if self.highest_winning_spread > other.highest_winning_spread{
			 Ordering::Less
		}
		else if self.highest_winning_spread < other.highest_winning_spread{
			 Ordering::Greater
		}
		else if self.games_played > other.games_played{
			 Ordering::Less
		}
		else if self.games_played < other.games_played{
			 Ordering::Greater
		}
		else {
			Ordering::Equal
		}
	}
}

impl PartialEq for Leader {
	fn eq(&self, other: &Leader) -> bool {
		self.player_name == other.player_name
		//self.percentage == other.percentage
	}
}

impl Eq for Leader {}

#[derive(Serialize, QueryableByName, Debug)]
pub struct TeamStats {
	#[sql_type = "Integer"]
	pub won: i32,
	#[sql_type = "Integer"]
	pub lost: i32,
	#[sql_type = "Integer"]
	pub played: i32,
    #[sql_type = "Float"]
	pub percentage: f32,
}

#[derive(Serialize, QueryableByName, Debug)]
pub struct VsStats {
    #[sql_type = "Varchar"]
	pub player_three: String,
    #[sql_type = "Varchar"]
	pub player_four: String,
	#[sql_type = "Integer"]
	pub won: i32,
	#[sql_type = "Integer"]
	pub lost: i32,
	#[sql_type = "Integer"]
	pub played: i32,
    #[sql_type = "Float"]
	pub percentage: f32,
}