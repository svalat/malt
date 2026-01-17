#![warn(missing_docs)]

//! Instrumenation library for MALT.

use std::io;
use std::fs::File;
use crate::json::rendering::json_state::JsonState;

/// Home made JSON handling implementation to keep low memory.
pub mod json;

/// Main entry point
fn main() {

	let mut file = io::BufWriter::new(File::open("output").unwrap());
	let _state = JsonState::new(& mut file, true, false);
	println!("Hello, world!");
}
