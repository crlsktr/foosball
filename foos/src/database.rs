use diesel::PgConnection;
use r2d2::{Pool, PooledConnection};
use r2d2_diesel::ConnectionManager;

pub struct ConnectionPool {
	pool: Pool<ConnectionManager<PgConnection>>,
}

impl ConnectionPool {
	pub fn create(connection_string: impl ToString) -> Result<Self, String> {
		let manager = ConnectionManager::<PgConnection>::new(connection_string.to_string());
		let pool = r2d2::Pool::builder()
			.build(manager)
			.map_err(|e| format!("Couldn't create the connection pool: {}", e))?;
		let connection_pool = ConnectionPool { pool };
		Ok(connection_pool)
	}

	pub fn get(&self) -> Result<Database, String> {
		let db = self.pool.get().map_err(|e| {
			format!(
				"Failed to get a connection from the connection pool:  {}",
				e
			)
		})?;
		let db = Database { db };
		Ok(db)
	}
}

pub struct Database {
	db: PooledConnection<ConnectionManager<PgConnection>>,
}

impl Database {
	pub fn connection(&self) -> &PgConnection {
		&*self.db
	}
}
