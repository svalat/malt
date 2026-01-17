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
	fn close_field(& mut self, _name: String)
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
	fn open_field(& mut self, name: String)
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

	/// Display a list separator.
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
}

/* 
	public:
		template <class T> void printField(const char* name, const T& value);
		template <class T> void printValue(const T & value, bool separator = true);
		template <class T> void printFieldArray(const char * name,const T * value,int size);
		template <class T> void printArray(const T * value,int size);
		void printFormattedField(const char * name,const char * format,...);
		void printFormattedValue(const char * format,...);
		void openFieldArray(const char * name);
		void closeFieldArray(const char * name);
		void openFieldStruct(const char * name);
		void closeFieldStruct(const char * name);
		void openArray(void);
		void closeArray(void);
		void openStruct(void);
		void closeStruct(void);
*/
//	private:
