use crate::schema::series;
use chrono::prelude::*;
use diesel::prelude::*;
use diesel::PgConnection;

#[derive(Queryable, QueryableByName, Serialize, Debug, Clone)]
#[table_name = "series"]
pub struct Series {
	pub id: i32,
	pub played_on: DateTime<Utc>,
	pub created_by: i32,
}

#[derive(Insertable, Deserialize)]
#[table_name = "series"]
pub struct NewSeries {
	pub played_on: DateTime<Utc>,
	pub created_by: i32,
}

impl Series {
	pub fn create(connection: &PgConnection, created_by: i32) -> Result<Series, String> {
		let new_series = NewSeries {
			played_on: Utc::now(),
			created_by,
		};

		let series_ = diesel::insert_into(series::table)
			.values(&new_series)
			.get_result(connection)
			.map_err(|e| format!("Couldn't create new_series: {}", e))?;

		Ok(series_)
	}
}
