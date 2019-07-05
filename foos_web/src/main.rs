use actix_web::{web, App, HttpServer};

mod config;
mod user;

use config::Config;
use foos::database::*;

fn main() {
	let config = config::FoosWebConfig::from_toml_file("web-config.toml");
	let _ = config.save("web-config.toml");

	if config.database_url.trim().to_string().is_empty() {
		println!("Must specify a database-url in the web-config.toml file");
		return;
	}

	let connection_pool: ConnectionPool = match ConnectionPool::create(config.database_url) {
		Ok(cp) => cp,
		Err(e) => {
			println!("Could not create connection pool: {}", e);
			return;
		}
	};

    HttpServer::new(move || {
        App::new()
			.data(connection_pool.clone())
			.route("/user/search", web::post().to(user::search_user))
			.route("/user/search/{term}", web::get().to(user::search_user_get))
    })
    .bind(&config.bind_url)
    .expect(&format!("Can not bind to {}", &config.bind_url))
    .run()
    .unwrap();
}

