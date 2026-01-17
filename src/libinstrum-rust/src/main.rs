use std::io;
use std::fs::File;
use crate::json::stream::json_state::JsonState;

pub mod json;

fn main() {

	let mut file = io::BufWriter::new(File::open("output").unwrap());
	let state = JsonState::new(& mut file, true, false);
	println!("Hello, world!");
}
