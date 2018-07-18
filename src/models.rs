use super::schema::games;

#[derive(Queryable, Insertable, Deserialize)]
#[table_name="games"]
pub struct Game {
    number: i32,
    round: String,
    date: String,
    teamone: String,
    teamtwo: String,
    scoreone: i32,
    scoretwo: i32,
}