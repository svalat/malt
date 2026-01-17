use std::fmt::Display;
use std::io;
use std::io::Write;

///
/// Define the current state for open/close checking.
/// @brief Enum to define the available json states.
///
#[derive(PartialEq, Clone, Copy)]
pub enum JsonStateEnum
{
	/// Currently on the root element.
	Root = 1,
	/// Currently inside structure, expect struct end or fields.
	Struct = 2,
	/// Currently inside array, expect array end or fields.
	Array = 4,
	/// Currently in state field, expect value, struct or array.
	Field = 8,
}

/// Short structure to build the state stack inside JsonState.
/// @brief Structure to follow the json state.
struct JsonStateStruct
{
	/** Current state **/
	state: JsonStateEnum,
	/** Used by structure and arrays to determine the needs of separator. **/
	is_first: bool,
}

/**
 * Stack of JsonStateStruct to follow the nested states while traversing the object 
 * and subobject fields. 
 * @brief Type to store the json states as a stack.
**/
type JsonStateStructStack = Vec<JsonStateStruct>;

/**
 * JsonState is the central class to convert a structure into json text format.
 * It ensure the storage of current conversion status to open/close arrays,
 * structures and to register entries. This class is used by the convertToJson() method.
 * @brief Class to help to export objects into Json.
**/
pub struct JsonState<'a>
{
	/** Indentation level. **/
	indent: i16,
	/** Stack of status to now in wich type of node we are. **/
	state_stack: JsonStateStructStack,
	/** Enable or disable indentation. **/
	use_indent: bool,
	/** Generate LUA instead of json (need to cleanup this integration). **/
	lua: bool,
	/** Use fast buffering stream */
	bufferd_stream: &'a mut io::BufWriter<dyn io::Write>,
}

/**
 * Trait to say that the type can be placed in a JSON stream
 */
pub trait JsonFieldValue {
	/**
	 * Convert the given type to streamed json format.
	 * @param state Define the json state to use for the conversion.
	 */
	fn convert_to_json<'a>(&self, state: &mut JsonState<'a>);
}

impl JsonState<'_> {
	/**
	 * Constructor of the JSonState class. It setup the state as JSON_STATE_ROOT
	 * and init indent to 0.
	 * @param out Define the output stream into which to write json output. NULL isn't supported here.
	 * @param indent If true, indent the output json code with tabulations, produce 
	 * compact json code.
	**/
	pub fn new(out: &'_ mut io::BufWriter<dyn io::Write>, indent: bool, lua: bool) -> JsonState<'_>
	{
		//init
		let mut state = JsonState {
			indent: 0,
			state_stack: JsonStateStructStack::new(),
			use_indent: indent,
			lua: lua,
			bufferd_stream: out
		};

		//push state
		state.push_state(JsonStateEnum::Root);

		return state
	}

	/// @return Return the current state for checking.
	pub fn get_state(&self) -> JsonStateEnum
	{
		return self.state_stack.last().unwrap().state;
	}

	/// Push the new state into the stack.
	/// @param state new state to init.
	fn push_state(&mut self, state: JsonStateEnum)
	{
		//create a new state
		let tmp = JsonStateStruct{
			state: state,
			is_first: true
		};

		//push it
		self.state_stack.push(tmp)
	}

	/// Check if a first element was provided in case of arrays or structure.
	/// Usefull to know if we need to add separators or not.
	fn first_is_done(& mut self)
	{
		self.state_stack.last_mut().unwrap().is_first = false;
	}

	/// Check if a first element was provided in case of arrays or structure.
	/// Usefull to know if we need to add separators or not.
	fn is_first(&self) -> bool
	{
		return self.state_stack.last().unwrap().is_first;
	}

	/// Pop the current state from the stack.
	/// @param state State to remove (only for checking).
	fn pop_state(& mut self, state: JsonStateEnum)
	{
		assert!(self.state_stack.last().unwrap().state == state);
		self.state_stack.pop();
	}

	
	/// Internal function to close the current field.
	/// @param name Name of the field to close (for checking).
	fn close_field(& mut self, _name: &str)
	{
		//check where we are
		assert!(self.get_state() == JsonStateEnum::Field);

		//setup state
		self.pop_state(JsonStateEnum::Field);

		//mark first as done
		self.first_is_done();
	}

	/// Internal function to start a new field.
	/// @param name Name of the field to declare.
	fn open_field(& mut self, name: &str)
	{
		//check where we are
		assert!(self.get_state() == JsonStateEnum::Root || self.get_state() == JsonStateEnum::Struct);

		//print name
		if !self.is_first()
		{
			if self.use_indent {
				writeln!(self.bufferd_stream, ",").unwrap();
			} else {
				write!(self.bufferd_stream, ",").unwrap();
			}
		}

		//setup state
		self.push_state(JsonStateEnum::Field);

		//print padding
		self.put_padding();

		//print name
		if self.lua {
			write!(self.bufferd_stream, "{}=", name).unwrap();
		} else {
			write!(self.bufferd_stream, "\"{}\":", name).unwrap();
		}
	}

	/// Write padding characters into output stream. It will use the local indent
	/// parameter and use tabulations.
	fn put_padding(& mut self)
	{
		if self.use_indent
		{
			//slow unbuffered version, but not for FastBufferedStream
			for _ in  0 .. self.indent {
				write!(self.bufferd_stream, "\t").unwrap();
			}
		}
	}

	/// Tells if we generate a LUA output instead of JSON.
	pub fn is_lua(&self) -> bool
	{
		return self.lua;
	}

	/**
	 * Display a list separator.
	**/
	pub fn print_list_separator(& mut self)
	{
		//check where we are
		assert!(self.get_state() as i8 & (JsonStateEnum::Field as i8 | JsonStateEnum::Array as i8 | JsonStateEnum::Root as i8) != 0);

		//separator
		if self.get_state() == JsonStateEnum::Array && !self.is_first() {
			write!(self.bufferd_stream, ", ").unwrap();
		}

		//mark done
		self.first_is_done();
	}

	/**
	 * Start a new array, mostly to be used internally or for the root element.
	**/
	pub fn open_array(&mut self)
	{
		//check where we are
		assert!(self.get_state() as i8 & (JsonStateEnum::Root as i8 | JsonStateEnum::Field as i8 | JsonStateEnum::Array as i8) != 0);

		//setup state
		self.push_state(JsonStateEnum::Array);

		//print name
		if self.lua {
			write!(self.bufferd_stream, "{{").unwrap();
		} else {
			write!(self.bufferd_stream, "[").unwrap();
		}
	}

	/**
	 * Close the current array. Mostly to be used internally or for the root element.
	**/
	pub fn close_array(&mut self)
	{
		//check where we are
		assert!(self.get_state() == JsonStateEnum::Array);

		//setup state
		self.pop_state(JsonStateEnum::Array);

		//print name
		if self.lua {
			write!(self.bufferd_stream, "}}").unwrap();
		} else {
			write!(self.bufferd_stream, "]").unwrap();
		}
	}

	/**
	 * Start a new structure. To be used internally, for root elements or for values inside arrays.
	**/
	pub fn open_struct(&mut self)
	{
		//check where we are
		assert!(self.get_state() as i8 & (JsonStateEnum::Field as i8 | JsonStateEnum::Array as i8 | JsonStateEnum::Root as i8) != 0);

		//setup state
		self.push_state(JsonStateEnum::Struct);

		//print name
		self.indent += 1;
		if self.use_indent {
			writeln!(self.bufferd_stream, "{{").unwrap();
		} else {
			write!(self.bufferd_stream, "{{").unwrap();
		}
	}

	/**
	 * Close the current structure. To be used internally, for root elements or for values inside arrays.
	**/
	pub fn close_struct(&mut self)
	{
		//check where we are
		assert!(self.get_state() == JsonStateEnum::Struct);

		//decr
		self.indent -= 1;

		//line break
		if self.is_first() == false && self.use_indent {
			writeln!(self.bufferd_stream, "").unwrap();
			self.put_padding();
		}

		//setup state
		self.pop_state(JsonStateEnum::Struct);

		//print name
		if self.use_indent {
			writeln!(self.bufferd_stream, "}}").unwrap();
		} else {
			write!(self.bufferd_stream, "}}").unwrap();
		}
		
		//padd
		self.put_padding();
	}

	/**
	 * Open a new field with struct as a value.
	 * @param name Name of the field.
	 */
	pub fn open_field_struct(&mut self, name: &str)
	{
		self.open_field(name);
		self.open_struct();
	}

	/**
	 * Close the field of type struct.
	 * @param name Name for check.
	 */
	pub fn close_field_struct(&mut self, name: &str)
	{
		self.close_struct();
		self.close_field(name);
	}

	/**
	 * Start a new field with array as content. Internal values must be declared
	 * with printFormattedValue() or printValue().
	 * It must be closed by closeFieldArray().
	 * @param name Name of the field to declare.
	**/
	pub fn open_field_array(&mut self, name: &str)
	{
		self.open_field(name);
		self.open_array();
	}

	/**
	 * Close a field array opened by openFieldArray().
	 * @param name Define the field to terminate (only for checking).
	**/
	pub fn close_field_array(&mut self, name: &str)
	{
		self.close_array();
		self.close_field(name);
	}

	/**
	 * Print field based on generic convertToJson methods to convert the given object into
	 * json. To be used into structures.
	 * @param name Name of the field to print.
	 * @param value Value to affecto the field.
	**/
	pub fn print_field(&mut self, name: &str, value: &dyn JsonFieldValue)
	{
		//print
		self.open_field(name);
		value.convert_to_json(self);
		self.close_field(name);
	}

	/**
	 * Print avalue based on generic convertToJson methods to convert the given object into
	 * json. To be used into arrays or for the root element.
	 * @param value Value to affecto the field.
	**/
	pub fn print_value(&mut self, value: &dyn JsonFieldValue)
	{
		self.print_value_sep_opt(value, true);
	}

	/**
	 * Print avalue based on generic convertToJson methods to convert the given object into
	 * json. To be used into arrays or for the root element.
	 * @param value Value to affecto the field.
	 * @param separator If need to put the separator of not
	**/
	pub fn print_value_sep_opt(&mut self, value: &dyn JsonFieldValue, separator: bool)
	{
		//check where we are
		assert!(self.get_state() as i8 & (JsonStateEnum::Field as i8 | JsonStateEnum::Array as i8 | JsonStateEnum::Root as i8) != 0);

		//separator
		if separator && self.get_state() == JsonStateEnum::Array && !self.is_first() {
			write!(self.bufferd_stream, ", ").unwrap();
		}

		//print
		value.convert_to_json(self);

		//done
		self.first_is_done();
	}

	/**
	 * Print C array as json array by using the generic template method convertToJson to
	 * convert each object into json.
	 * @param name Name of the field to print.
	 * @param value Base address of the values array to print.
	 * @param size Number of values into array.
	**/
	pub fn print_field_array(&mut self, name: &str, value: &mut dyn Iterator<Item = &dyn JsonFieldValue>)
	{
		//check where we are
		assert!(self.get_state() as i8 & (JsonStateEnum::Struct as i8 | JsonStateEnum::Root as i8) != 0);

		//print
		self.open_field(name);
		self.open_array();
		value.for_each(|x| self.print_value(x));
		self.close_array();
		self.close_field(name);
	}

	/**
	 * Print C array as json array by using the generic template method convertToJson to
	 * convert each object into json.
	 * @param value Base address of the values array to print.
	 * @param size Number of values into array.
	**/
	pub fn print_array(&mut self, value: &mut dyn Iterator<Item = &dyn JsonFieldValue>)
	{
		//check where we are
		assert!(self.get_state() as i8 & (JsonStateEnum::Field as i8 | JsonStateEnum::Array as i8 | JsonStateEnum::Root as i8) != 0);

		//open
		self.open_array();

		//fill
		value.for_each(|x| self.print_value(x));

		//close
		self.close_array();
	}

	/**
	 * Push raw value escapted as a string in the stream. This function is
	 * done to push low level types to the stream.
	 */
	pub fn push_raw_string<T: Display>(&mut self, value: &T)
	{
		//check where we are
		assert!(self.get_state() as i8 & (JsonStateEnum::Field as i8 | JsonStateEnum::Array as i8 | JsonStateEnum::Root as i8) != 0);

		//push
		write!(self.bufferd_stream, "\"{}\"", value).unwrap();
	}

	/**
	 * Push raw value escapted as a string in the stream. This function is
	 * done to push low level types to the stream.
	 */
	pub fn push_raw_string_str(&mut self, value: &str)
	{
		//check where we are
		assert!(self.get_state() as i8 & (JsonStateEnum::Field as i8 | JsonStateEnum::Array as i8 | JsonStateEnum::Root as i8) != 0);

		//push
		write!(self.bufferd_stream, "\"{}\"", value).unwrap();
	}

	/**
	 * Push raw value in the stream. This function is
	 * done to push low level types to the stream.
	 */
	pub fn push_raw_value<T: Display>(&mut self, value: &T)
	{
		//push
		write!(self.bufferd_stream, "\"{}\"", value).unwrap();
	}

	/**
	 * Push raw value in the stream. This function is
	 * done to push low level types to the stream.
	 */
	pub fn push_raw_value_str(&mut self, value: &str)
	{
		//push
		write!(self.bufferd_stream, "\"{}\"", value).unwrap();
	}
}

impl JsonFieldValue for String
{
	fn convert_to_json(&self, state: &mut JsonState)
	{
		state.push_raw_string(self);
	}
}

impl JsonFieldValue for str
{
	fn convert_to_json(&self, state: &mut JsonState)
	{
		state.push_raw_string_str(self);
	}
}

impl JsonFieldValue for bool
{
	fn convert_to_json(&self, state: &mut JsonState)
	{
		if *self {
			state.push_raw_value_str("true");
		} else {
			state.push_raw_value_str("false");
		}
	}
}

impl JsonFieldValue for f32
{
	fn convert_to_json(&self, state: &mut JsonState)
	{
		state.push_raw_string(self);
	}
}

impl JsonFieldValue for f64
{
	fn convert_to_json(&self, state: &mut JsonState)
	{
		state.push_raw_string(self);
	}
}

impl JsonFieldValue for i8
{
	fn convert_to_json(&self, state: &mut JsonState)
	{
		state.push_raw_string(self);
	}
}

impl JsonFieldValue for i16
{
	fn convert_to_json(&self, state: &mut JsonState)
	{
		state.push_raw_string(self);
	}
}

impl JsonFieldValue for i32
{
	fn convert_to_json(&self, state: &mut JsonState)
	{
		state.push_raw_string(self);
	}
}

impl JsonFieldValue for i64
{
	fn convert_to_json(&self, state: &mut JsonState)
	{
		state.push_raw_string(self);
	}
}

impl JsonFieldValue for i128
{
	fn convert_to_json(&self, state: &mut JsonState)
	{
		state.push_raw_string(self);
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn constructor() {
		let out = Vec::<u8>::new();
		let _state = JsonState::new(&mut io::BufWriter::new(out), true, false);
	}

	#[test]
	fn basic_empty() {
		let mut stream = io::BufWriter::new(Vec::<u8>::new());
		let mut state = JsonState::new(&mut stream, true, false);
		state.open_struct();
		state.close_struct();
		let as_str = str::from_utf8(stream.buffer()).unwrap();
		assert_eq!(as_str, "{\n}\n");
	}

	#[test]
	fn basic_print_field_str() {
		let mut stream = io::BufWriter::new(Vec::<u8>::new());
		let mut state = JsonState::new(&mut stream, true, false);
		state.open_struct();
		state.print_field("test", &"toto".to_string());
		state.close_struct();
		let as_str = str::from_utf8(stream.buffer()).unwrap();
		assert_eq!(as_str, "{\n\t\"test\":\"toto\"\n}\n");
	}

	#[test]
	fn basic_print_two_fields_str() {
		let mut stream = io::BufWriter::new(Vec::<u8>::new());
		let mut state = JsonState::new(&mut stream, true, false);
		state.open_struct();
		state.print_field("test", &"toto".to_string());
		state.print_field("test2", &"toto2".to_string());
		state.close_struct();
		let as_str = str::from_utf8(stream.buffer()).unwrap();
		assert_eq!(as_str, "{\n\t\"test\":\"toto\",\n\t\"test2\":\"toto2\"\n}\n");
	}

	#[test]
	fn basic_print_field_numbers() {
		let mut stream = io::BufWriter::new(Vec::<u8>::new());
		let mut state = JsonState::new(&mut stream, true, false);
		state.open_struct();
		state.print_field("i8", &(8 as i8));
		state.print_field("i16", &(16 as i16));
		state.print_field("i32", &(32 as i32));
		state.print_field("i64", &(64 as i64));
		state.print_field("f32", &(32.1 as f32));
		state.print_field("f64", &(64.1 as f64));
		state.close_struct();
		let as_str = str::from_utf8(stream.buffer()).unwrap();
		assert_eq!(as_str, "{\n\t\"i8\":\"8\",\n\t\"i16\":\"16\",\n\t\"i32\":\"32\",\n\t\"i64\":\"64\",\n\t\"f32\":\"32.1\",\n\t\"f64\":\"64.1\"\n}\n");
	}
}

/*******************  FUNCTION  *********************/
//specific implementations for some known types
/*
void convertToJson(JsonState & json, bool value);
void convertToJson(JsonState & json, void * ptr);
void convertToJson(JsonState & json, const htopml::IJsonConvertible & object);
void convertToJson(JsonState & json, htopml::IJsonConvertible & object);
*/

/*******************  FUNCTION  *********************/
//generic version
//template <class T> void convertToJson(JsonState & json, const T & iterable);

/*******************  FUNCTION  *********************/
//specific implementation for some STL containers
/*
template <class T> void convertToJson(JsonState & json, const std::vector<T> & iterable);
template <class T> void convertToJson(JsonState & json, const std::list<T> & iterable);
template <class T,class U> void convertToJson(JsonState & json, const std::map<T,U> & iterable);
template <class U> void convertToJson(JsonState & json, const std::map<std::string,U> & iterable);
*/
