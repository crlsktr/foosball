use crate::schema::users;
use diesel::prelude::*;
use diesel::PgConnection;

#[derive(Serialize, Deserialize, Debug)]
pub struct BasicUser {
	pub id: i32,
	pub username: String,
}

#[derive(Identifiable, Queryable, Deserialize, Serialize, Clone)]
#[table_name = "users"]
struct User {
	pub id: i32,
	pub username: String,
	pub password: String,
	pub enabled: bool,
}

#[derive(Insertable)]
#[table_name = "users"]
struct NewUser<'a> {
	pub username: &'a str,
	pub password: &'a str,
	pub enabled: bool,
}

pub fn create_user(
	connection: &PgConnection,
	username: &str,
	password: &str,
) -> Result<BasicUser, String> {
	let hash = bcrypt::hash(password, bcrypt::DEFAULT_COST)
		.map_err(|_e| "Password hashing failed.".to_string())?;

	let new_user = NewUser {
		username,
		password: &hash,
		enabled: true,
	};

	let user: User = diesel::insert_into(users::table)
		.values(&new_user)
		.get_result(connection)
		.map_err(|e| format!("Couldn't create user: {}", e))?;

	let user = BasicUser {
		id: user.id,
		username: user.username,
	};
	Ok(user)
}

pub fn authenticate(
	connection: &PgConnection,
	username: &str,
	password: &str,
) -> Result<BasicUser, String> {
	let user = get_user(connection, username)?;
	let verified = bcrypt::verify(password, &user.password)
		.map_err(|_e| "Couldn't verify username or password".to_string())?;

	if !verified {
		return Err("Couldn't verify username or password".to_string());
	}
	let user = BasicUser {
		id: user.id,
		username: user.username,
	};
	Ok(user)
}

pub fn search(connection: &PgConnection, search: &str) -> Result<Vec<BasicUser>, String> {
	use users::dsl as u;
	let search = format!("%{}%", search);
	let results = users::table
		.filter(u::username.like(&search))
		.load::<User>(connection)
		.map_err(|e| format!("Searching failed: {}", e))?;

	let basic_users = results
		.iter()
		.map(|u| BasicUser {
			id: u.id,
			username: u.username.clone(),
		})
		.collect();
	Ok(basic_users)
}

fn get_user(connection: &PgConnection, username: &str) -> Result<User, String> {
	use users::dsl as u;
	let user = users::table
		.filter(u::username.eq(username))
		.first::<User>(connection)
		.map_err(|e| {
			format!(
				"Couldn't load a current user with username {}: {}",
				username, e
			)
		})?;
	Ok(user)
}
