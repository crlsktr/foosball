use crate::schema::series;
use chrono::prelude::*;
use diesel::prelude::*;
use diesel::PgConnection;

#[derive(Queryable, QueryableByName, Serialize, Debug, Clone)]
#[table_name = "series"]
pub struct Series {
	pub id: i32,
	pub played_on: DateTime<Utc>,
}

#[derive(Insertable, Deserialize)]
#[table_name = "series"]
pub struct NewSeries {
	pub played_on: DateTime<Utc>,
}

impl Series {
	pub fn create(connection: &PgConnection) -> Result<Series, String> {
		let new_series = NewSeries {
			played_on: Utc::now(),
		};

		let series_ = diesel::insert_into(series::table)
			.values(&new_series)
			.get_result(connection)
			.map_err(|e| format!("Couldn't create new_series: {}", e))?;

		Ok(series_)
	}

	// pub fn find(connection: &PgConnection, id: i32) -> Result<Series, String> {
	// 	use series::dsl as s;

	// 	let series_: Series = s::series
	// 		.find(id)
	// 		.first::<Series>(connection)
	// 		.map_err(|e| format!("Unable to find match: {}", e))?;

	// 	Ok(series_)
	// }
}
