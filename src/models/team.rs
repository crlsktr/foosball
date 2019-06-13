#![allow(dead_code)]
use super::super::schema::teams;

#[derive(Queryable, Serialize, Debug)]
pub struct Team {
    pub id: i32,
    pub player_one_id: i32,
    pub player_two_id: i32,
}

#[derive(Insertable, Deserialize)]
#[table_name="teams"]
pub struct NewTeam {
    pub player_one_id: i32,
    pub player_two_id: i32,
}