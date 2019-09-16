use clap::{App,  SubCommand};
use foos::database::Database;

pub fn get_replay_command<'a, 'b>(author: &'a str, version: &'a str) -> App<'a, 'b> {
    let replay_sub = SubCommand::with_name("replay")
		.about("replay game results and history records")
		.version(version)
		.author(author);
    
    replay_sub
}

pub fn entry(db: &Database) {
	match foos::replay_ranking(db.connection()) {
        Ok(_) => { println!("done replaying")},
        Err(e) => { println!("error replaying: {}", e)}
    };
}