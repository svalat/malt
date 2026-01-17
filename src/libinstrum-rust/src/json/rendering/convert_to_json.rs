use std::io;
use super::json_state::JsonState;
use super::json_state::JsonFieldValue;

/**
 * Generic implementation for the final user to convert an object to json document.
 * To be supported, each object of you tree need to provide standard output stream operator
 * or implement a specific version of convertToJson(JsonState &,T & value).
 * @param out Define the output stream into which to print the json output.
 * @param value Reference to the object to convert.
**/
pub fn convert_to_json<T>(out: &'_ mut io::BufWriter<dyn io::Write>,value: &T, indent: bool)
where
	T: JsonFieldValue
{
	let mut state = JsonState::new(out, indent, false);
	state.print_value(value);
}

/**
 * Generic implementation for the final user to convert an object to json document.
 * To be supported, each object of you tree need to provide standard output stream operator
 * or implement a specific version of convertToJson(JsonState &,T & value).
 * @param out Define the output stream into which to print the json output.
 * @param value Reference to the object to convert.
**/
pub fn convert_to_lua<T>(out: &'_ mut io::BufWriter<dyn io::Write>,value: &T, indent: bool)
where
	T: JsonFieldValue
{
	let mut state = JsonState::new(out, indent, true);
	state.print_value(value);
}
