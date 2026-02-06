/* This is me trying Rust.

- "Everything in Rust is designed to blame you! :)"

- Started using new extensions for a better programming and debugging of my Rust stack. Here is a small desc:
	- Custom 'settings.json'
	- TODO: Any comment that has "TODO" in codebases is highlighted (customized in settings.json too lol)
	- FIXME: "FIXME" too
	- Error lens will show me any compiler errors (and Rustc throws a lot) inline (without having to hover)
	- ...
*/

#![allow( non_snake_case )]

use std::io;

#[allow( dead_code )] // Prevents Rust warnings if the function isn't used
/*
We can also enforce global lints in the Cargo.toml of the project file:

[lints.rust]
unsafe_code = "forbid"
unused_mut = "warn"
missing_docs = "allow"

The supported levels are:
	* allow
	* warn
	* deny
	* forbid
*/
fn vars_and_data_types() {
	println!( "\nHello World, Hello Rust!\n" );

	/*
	Integers.

	- Integer data types have bit lengths of type signed or unsigned: 8, 16, 32, 64. Signed integers are marked with i, and u for unsigned.
	- Their difference is that unsigned integer data types cannot be negated but can contain larger values!
	*/

	println! ( "---Integers---\n" ); // The 'println!' macro invokes a '\n' by default
	let x: i8 = -25; // Variables are immutable by default, i.e. value (or data type ...Python...) cannot be changed!
	let y: u8 = 25;
	println! ( "This is a signed 8-bit integer: {}", x );
	println! ( "This is an unsigned 8-bit integer: {}\n", y );

	/*
	Floats.

	- Some bit sizes aren't 'stable' in rust's compiler for float data types, and hence cannot be used. Recommend using just f32 or f64.
	*/

	println! ( "---Floats---\n" );
	let fx: f64 = 255.0;

	println!( "This is a 64-bit float: {:?}", fx );

	// Casting: Converting certain, primitive data types to other data types (like floats to integers...)
	let csum: u8 = fx as u8 - 5; // Convert the f64 var to an unsigned, 8-bit integer and subtract 5 from it

	println!( "This is a f64 to u8 casted float: {:?}\n", csum );

	/*
	Booleans and Immutability.

	- We can make a varibale mutable by supplying a simple 'mut' after let.
	*/

	println! ( "---Booleans---\n" );
	let learning_yesterday: bool = true; // This is immutable by default

	println! ( "This is an immutable bool value: {:?}", learning_yesterday );

	let mut learning_today: bool = false;

	println! ( "This is a mutable bool value before: {}", learning_today ); // Rust throws an error if a mutable object is changed before being used!
	learning_today = false;
	println! ( "This is a mutable bool value after: {}\n", learning_today );

	/*
	Strings and Chars (Standalone Characters).

	- Strings and Chars are differentiated by their quote type, i.e. single quotes for chars and double quotes for strings.
	- String variables are string pointers by default. String Slices (&str - default) is an immutable reference/slice to a string while String is a mutable, growing string type.

	More Technical Details:
	- A String is a heap allocated, growable, UTF-8 encoded string, and is of 'owned type'. This means the variable owns the value of the underlying data, and is responsible for cleaning it up (i.e. when the variable goes out of scope, the underlying data is automatically deallocated). This allows it to be efficient for string manipulation.
	- A String Slice on the other hand, is a view into a string. It represents a contigous sequence of UTF-8 encoded bytes, making it efficient for read-only operations and is of 'borrowed type'. This means that it doesn't own the underlying data, it simply has access to it.
	- String Slices also have another variant called String Literals, where they can have a static lifetime, meaning the data being pointed to is guaranteed to be available for the entire duration of the program's execution. Syntax: 'let hello: &'static str = "Hello World!";'. Mainly used in structs/enums.

	- A String type is composed of a `ptr` (the pointer), `len` (the length of the data within), and `cap` (the capacity of the String, with extra allocated space for future expansion).
		- E.g. `let monster = String::from( "Goblin" );`. The String's pointer ..., the len is `6`, and the capacity will be `6 + n` (the extra space allocated).

	Others String Types ~ Covered in detail later:
	A. Specialized Strings
		- &mut str: Creates a mutable reference to a str.

	B. Strings for Interoperability
		- OsString and OsStr: Useful for handling multi-platform strings (for compatability).
		- Path and PathBuf: Strings that handle file paths differently for different platforms.
		- CStr and CString: Useful when interfacing Rust code with C libraries that expect null terminated strings.
	*/

	println! ( "---Strings and Characters---\n" );
	let c = 'A';
	let s = "Name";
	let s1 = String::from( "Name" ); // This is a String data type

	println! ( "This is a char: {:?}, This is a str: {:?}\n", c, s  ); // The ':?' (debug output) inside the curly braces gives the 'println' macro extra formatting options, like including the single or double quotes for the data types.
	println! ( "This is also a string: {}", s1 );

	/*
	- We can also supply '.unwrap()' for types to extract the value contained within the types and formatted simply.
	- At the same time, using unwrap() is considered bad practice in most production code due to its behavior when encountering an empty Option (i.e. None) or an error Result (i.e. Err). When unwrap() is called on a None or Err value, it causes the program to panic, which results in the program immediately terminating.
	*/

	// Concatenation (different for Strings and String Slices)
	let mut a_string: String = String::from( "Hello" );

	a_string.push_str( "World" ); // This accepts only &str types, and 'push()' for chars

	/*
	Tuples, Arrays, and Vectors (covered later).
	
	- They are types used to store multiple values, but differ in terms of type, uniformity, size mutability, memory layout, and capabilities.

	A. Array
	- Fixed size at compile time.
	- Homogenous: All elements must be the same type.
	- Stored on the stack (or boxed heap).
	- Syntax: 'arr = []'.
	- Arrays are always immutable, can't be dynamically resized, and can't change the size after it's defined.

	B. Tuples
	- Fized size.
	- Heterogenous.
	- Also stored on the stack.
	- Accessed via indexing its elements.
	- Syntax: 'tup = ()'.
	- Iterating over a tuple isn't easy (need destructuring).
	- Not dynamically sized or resizable.
	- Elements can be mutable individually.

	- Tuples and Arrays are called 'Compound Types'.

	C. Vectors (Dynamic Arrays)
	- Dynamic size: We can add/remove elements at runtime.
	- Homogeneous.
	- Stored on the heap.
	- Automatically handles memory allocation.
	- Slight performance cost vs Arrays due to heap allocation and dynamic resizing.
	- Requires importing from the standard library, 'Vec'.
	- Can be mutable.
	- Syntax: 'vect = vec![]' or 'vect = Vec::new();'

	- Vectors (along with other types covered later) are called 'Collections'.

	Quick Decision Guide:
	- Use an Array if you know the number and type of items at compile time.
	- Use a Tuple for grouping a small number of values of different types.
	- Use a Vector if you need a dynamic list of items of the same type.
	*/

	println! ( "---Tuples and Arrays---\n" );
	let tup = ( "Trevor", "Sullivan", 40 );

	println! ( "This is a tuple: {:?}", tup ); // Tuples that have different data type values cannot be printed without the default formatter, i.e. cannot run without supplying the ':?'

	let arr: [u16; 6] = [1, 2, 3, 4, 5, 6];
	println! ( "This is an array of u16 integers: {:?}\n", arr );

	/*
	Slices: Data types that point to arrays*.
	*/

	println! ( "---Slices---" );
	let sl = &arr[1..=4]; // The '..' specifies a range of elements to include, in this case values from index 1 (m) to 4 (n). Optionally, we can add the '=' to include the n-th element (excluded by default)

	println! ( "Original array: {:?}", arr );
	println! ( "This is a slice of index range 1-4 (including 4): {:?}\n", sl );

	// Unit type: a data type with an empty value
	let e: () = ();

	println! ( "This is a unit type: {:?}\n", e );

	// Taking string value from input
	println! ( "---Inputs---" );
	let mut name1 = String::new(); // Create an empty string or from("")

	/*
	Differences between new() and from(): new is used to initialize an empty string, while from is used to initialize and assign a default, immutable value to the string (both are different from &str).
	*/

	println! ( "Enter your name:" );

	io::stdin()
		.read_line( &mut name1 ) // The string type must be converted to a string pointer type (since String::new() or String::from("") is used)
		.expect( "" ); // Recommended for safety
	// This "keyword structure" is called "The Builder Pattern"

	let name1 = name1.trim(); // Remove the extra '\n' that is concatenated when receiving inputs with 'io::stdin()'
	// or by adding 'replace( "\n", "" )' to 'io::stdin()' (recommended)

	println! ( "Hello, {}!", name1 );

	// Alternate 01
	println! ( "Enter your name:" );

	let name2 = &mut String::new();
	io::stdin()
		.read_line( name2 )
		.expect( "..." );

	let name2 = name2.trim();

	println! ( "Hello {}!", name2 );

	// Or with 'unwrap()': 'io::stdin().read_line( var ).unwrap();'

	/*
	Consts (not covered here).

	- In Rust, const is used to define constants, which are named values that must be explicitly types and have a static lifetime. Unlike variables, consts are immutable and cannot be modified after init.
	- Syntax: 'const MESSAGE: &str = "Hello World!";'

	- Rust also has a different constant declaration type called static. While const represents compile-time constraints (which are inlined at each usage) and are not stored on memory as variables (their values are directly substituted at compile time where they are used), static variables have a fixed memory location in the program.
	- On the other hand, static variables ('static TEXT: &str = "Hello World!";') can be declared as either immutable or mutable, but mutable statics required 'unsafe' for access. They are often used for interfacing with C code. They are considered unsafe and can cause concurrency issues.
	*/
}

// Ownership example
#[allow( dead_code )]
pub fn test_ownership() -> () {
	let s1 = String::from( "This has a value!" ); // The variable owns the value (Strings)!
	let s2 = s1; // The s2 variable moved its value, and is now responsible for it (Moves ownership, and s1 is no longer valid)

	// println! ( "{:?}", s1 ); // This will cause an error

	println! ( "{:?}", s2 );

	// Borrowing...
	let s3 = &s2; // This creates a "reference" to s2's value (what we commonly refer to "pointers")

	println! ( "{:?} and {:?}", s2, s3 ); // Now both of them can be used, since s3 borrowed s2's value (by pointing to it)

	// Cloning (copying the data within works on integers and such)
	let s4 = s2.clone(); // Creates a deep copy of the value, allowing us to have a completely independent instance
	// Cloning can be expensive than borrowing (copy), as it involves allocating new mem and copying the data. Use it when you need a distinct copy of the data.

	println! ( "{:?} and {:?}", s2, s4 ); // Again, both of them can be used
}

/*
- Lifetime specifiers are a way to indicate how long references to data are valid. They help the Rust compiler ensure memory safety by preventing dangling references, data races, and other common issues related to mem mgmt.
- In the case of the MutPerson struct, our code instructs the compiler that the Something attr will have a lifetime persistency parallel to its struct.
- Apart from the "static" specifier, all letters (small or capital) are generic lifetime specifiers, and can be used anywhere.
- Types can also have multiple lifetime specifiers!.
*/

#[allow( dead_code )]
fn get_name( first_name: &str, last_name: &str ) -> String { // Rust enforces us to return the specific data type if we declare it
	// String concatenation: takes types as inputs and returns a formatted string
	let full_name = format! ( "{0} {1}", first_name, last_name );

	return full_name;
}

pub mod helpers; // Call a module (as a public mod, making its contents visible)
pub mod calc;

/*
- To call a specific component from a module: 'mod module; use module::component;'. Therefore, 'component' can be used without supplying the extra 'module::component'... (recommended).
- And to call from a folder:
	* Approach one: 'mod folder;' with a 'mod.rs' inside the folder.
	* Approach two: Creating a 'folder.rs' file in src, a folder with the same name in src, then adding 'pub mod file;' in the folder.rs file, with 'file.rs' inside the folder. Then we can simply access the sub-modules by calling 'pub folder;'in the main.rs src.
*/

#[allow( dead_code )]
fn functions_and_modules() {
	// Calling a function
	vars_and_data_types();

	// Calling a function with arguments
	let name1 = get_name( "Shane", "Jones" );

	println! ( "Hello, {} (from inline function)!", name1 );

	// Calling a function from a module
	let name2 = helpers::get_name_mod( "Shane", "Jones" );
	println! ( "Hello, {} (from mod function)!", name2 );

	// Calling a function from a module's child module
	let name3 = helpers::namehelpers::get_name_child_mod( "Shane", "Jones" );
	println! ( "Hello, {} (from public child mod function)!", name3 );

	let age = helpers::agehelpers::add_5_to_age( 42 );
	println! ( "New age is: {} (from child module public function)!", age );

	// Calling functions from another module
	println! ();
	let sum = calc::calc_sum( 25.0, 32.0 );
	let diff = calc::calc_diff( 100.0, 46.0 );
	let prod = calc::calc_prod( 2.2, 99.2 );
	let res = calc::calc_res( 100.0, 100.0 );

	println! (
		"Results are: {}, {}, {}, {}", sum, diff, prod, res
	);
}

// Control flows
#[allow( dead_code )]
fn test_if() {
	let age: u8 = 16;

	let age_input = &mut String::new();

	println! ( "How old are you?" );
	io::stdin().read_line( age_input ).unwrap();

	let age_c = age_input.replace( "\n", "" ).parse::<u8>().unwrap(); // Parse is used to convert strings to other data types (and we need to remove the '\n' before parsing)

	if age_c > age {
		println! ( "User is old enough to drive!" );
	} else if age_c == 16 || age_c == 15 {
		println! ( "User is almost old enough to drive!" );
	} else {
		println! ( "User is not old enough to drive!" );
	}

	// Simple conditionals
	let drivers_license: bool = if age_c > 16 { true } else { false };

	println! ( "User has driver license? {}", drivers_license );
}

#[allow( dead_code )]
fn test_while() {
	let age: u8 = 16;
	let mut c_age: u8 = 0;

	while c_age < age {
		println! ( "Waiting..." );
		c_age += 1;

		if c_age == 6 { break; }
	}
}

#[allow( dead_code )]
fn test_loop() { // Kind of useless
	let mut x: u8 = 1;

	loop {
		println! ( "Waiting..." );

		if x > 5 { break; }

		x += 1;
	}
}

#[allow( dead_code )]
fn test_for() {
	let arr = [18, 16, 41, 35];

	// Iterating through the array with 'for' and running an 'if' loop for case matching* (pretty simple)
	for x in arr {
		if x > 16 {
			println! ( "User is eligible to drive!: '{}'", x );
		} else {
			println! ( "User is not eligible to drive!: '{}'", x );
		}
	}
}

pub mod closures;

// Matches and match patterns *not fully covered*
#[allow( dead_code)]
fn test_match_int() {
	/*
	Formatting match cases:
	
	1.
		case1 => {...;}
		case2 => {...;}
		_ => {...}

	2. Recommended
		case1 => ...,
		case2 => ...,
		case3 => ...
	*/

	let myage: u16 = 5;

	match myage { // Run comparisons against the supplied variable ~ Literal pattern
		35 => {
			println! ( "This is a match!" );
		}
		_ => {
			println! ( "This isn't a match!" );
		}
	} // Used the first matching type to illustrate

	match myage { // Range pattern
		1..=35 => println! ( "This is within match!" ), // Between 1 and 35, including 35
		50.. => println! ( "This is above the match!" ), // 50 and above
		_ => println! ( "This isn't a match!" ) // The range between 35 and 50 is mapped here by defintion (unless defined explicity for those ranges: '0 => {...} and 36..=29 {...}')
	}

	// match myage { // Grouped pattern
	// 	1 | 5 => println! ( "This is a match!" ),
	// 	_ => println! ( "This isn't a match!" )
	// }
}

/*
Enums.

- A powerful feature that allow us to define a type that can be of several different variations. Each variant can also contain data of different types, making them very flexible and useful for representing complex data structures.

Option Enum Types.
- They represent an optional value Some<T>, that contains a value, or None. They can be used for:
	* Initial values
	* Return types for funcs
	* Return types for reporting simple errors, where None is returned
	* Optional function args
	* Nullable pointers
	* Swapping things out of difficult situations
	* Optional struct fields...!!!
	* Struct fields that can be loaned or "taken"
- Custom enum types can also be used with Optional enums, and as return types too.
*/

// Option enums as return types for funcs
#[allow( dead_code, unused_assignments )]
fn test_op_int() -> Option<u8> {
	let mut op1: Option<u8> = None; // Initialize an option type var to a null value
	op1 = Some( 10 );

	return op1;
}

#[allow( dead_code )]
#[allow( unused_assignments )]
fn test_op_str() -> Option<String> {
	let mut op: Option<String> = None;
	op = Some( "Name".to_string() );

	return op;
}

// A custom enum defintion
#[allow( dead_code )]
enum Direction {
	North,
	South,
	East,
	West
}

// A function that uses an enum as an optional arg
#[allow( dead_code )]
fn test_op_directions( direction: Direction ) { // i.e. the arg can only be a variant from 'Direction'
	match direction { // Used with a simple match case
		Direction::North => println! ( "This is the north direction" ),
		Direction::South => println! ( "This is the south direction" ),
		Direction::East => println! ( "This is the east direction" ),
		Direction::West => println! ( "This is the west direction" )
		// No default match case ('_ => {...}'), since all match patterns are defined ...
	}
}

// Enums with different data type variants
#[allow( dead_code )]
enum Shape { // Each variant will have its own input data type
	Circle( f64 ),
	Rectangle( f64, f64 )
}

// A function that uses an enum as an optional arg, has variants that take their own args, and an Option return type (optional)
#[allow( dead_code )]
fn test_op_shapes( shape: Shape ) -> Option<f64> { // It can either return a float or a None
	match shape {
		Shape::Circle( radius ) => return Some( std::f64::consts::PI * radius * radius ),
		Shape::Rectangle( length, width ) => return Some( length * width )
	}
}

#[allow( dead_code )]
fn test_op_shapes_check() {
	let circle = Shape::Circle( 5.0 );
	let rect = Shape::Rectangle( 22.0 ,  10.0 );

	println! ( "Circle area: {:?}", test_op_shapes( circle ).unwrap() );
	println! ( "Circle area: {:?}", test_op_shapes( rect ).unwrap() );
}

/*
Methods on enums.

- Enums can *also* have functions (called methods when implemented). Their methods can either be directly mapped on the enum itself, mapped from a custom trait, or from an existing trait (e.g. 'ToString').
- Traits are a way to define shared behavior that types can implement (like interfaces from other programming languages). Each type can implement them in their own way.
*/

// Implement methods directly to the Direction enum
#[allow( dead_code )]
impl Direction {
	fn check_if_vertical( direction: Direction ) -> Option<bool> {
		match direction {
			Direction::North | Direction::South => return Some( true ),
			_ => return Some( false )// No need to explicitly define the other cases
		}
	}
}

// Implement methods from a custom trait
#[allow( dead_code )]
trait Area {
	fn area( &self ) -> f64;
}

impl Area for Shape { // Implements the Area trait for the enum Shape
	fn area( &self ) -> f64 {
		match self {
			Shape::Circle( radius ) => {
				return std::f64::consts::PI * radius * radius;
			}

			Shape::Rectangle( width , length ) => {
				return width * length;
			}
		}
	}
}

// Implement a method from a standard trait
impl ToString for Shape {
	fn to_string( &self ) -> String { // The method must be inside the trait!
		match self { // The method converts the areas of the shapes to strings
			Shape::Rectangle( width, length ) => return ( width * length ).to_string(),
			Shape::Circle( radius ) => return ( std::f64::consts::PI * radius * radius ).to_string()
		}
	}
}

// Use the implementations
#[allow( dead_code )]
fn test_direct_impl() -> () {
	let direction = Direction::North;
	println! ( "{:?}", Direction::check_if_vertical( direction ).unwrap() );
}

#[allow( dead_code )]
fn test_trait_impl() -> () {
	let shape = Shape::Rectangle( 1.0 , 22.0 );
	let shape2 = Shape::Circle( 22.0 );

	println! ( "{}", Shape::area( &shape ));
	println! ( "{:?}", Shape::to_string( &shape2 ) ); // Used the debug formatter to show the appropriate data type
}

// Enums with variants having attributes -> Structured Enums.
// TODO: Not sure how to use this!
pub enum SomeEnum {
	Something {
		x: f64,
		y: f64
	}
}

// We can also use implementations as function return types by supplying 'impl Trait' as the return type. This mean the function returns a type that implements the trait...

/*
Structs.

- A struct is a custom data type that lets us create complex data structures. They are similar to classes in other prog languages but are primarily used to group related data together, creating blueprints for objects and their data attributes.
- Methods are not allowed in structs by default, and can only be declared in 'impl' blocks.
- Additionally, if structs are being called from a module, note that their attributes must also be set to public if we want them to be visible to the caller alongside the structs*.
- Structs are also immutable by default, but can be mutable if the instance is mutable or by defining the attr using a 'Cell<T>', which implements interior mutability (implemented later).
- Structs can also be return types to functions and types for enum variants:
*/

// Simple illustration
#[allow( dead_code )]
enum Pos { X, Y }
#[allow( dead_code, non_snake_case )]
struct Dir { X: Pos, Y: Pos }
#[allow( dead_code )]
enum SecondPos { Pos( Dir ) }

#[allow( non_snake_case )] // Rust enforces 'snake case' (more like a recommendation with a warning) variable names and can be disabled by supplying this attr
#[derive( Debug )] // Traits need this to work in the debug  'println!' formatter
struct  Person {
	Name: String,
	Age: u16
}

// Apply implementations to the struct
impl Person {
	fn get_full_details( &self ) {
		println! ( "User's name is {} and is {} years old!", self.Name, self.Age );
	}
}

// Structs with custom attribute types (enums) ~ Helpers to the Vehicle struct
#[allow( dead_code )]
#[derive( Debug )]

enum VehicleMan {
	Toyota,
	Mercedes,
	Volks
}

#[allow( dead_code )]
#[derive( Debug )]
enum VehicleColor {
	Red,
	Silver,
	Blue,
	Black,
	White,
	Green
}

#[allow( dead_code )]
#[derive( Debug )]
struct Vehicle {
	manufacturer: VehicleMan,
	model: String,
	color: VehicleColor,
}

#[allow( dead_code )]
fn new_vehicle(
	man: VehicleMan,
	model: String,
	color: VehicleColor
) -> Vehicle { // Defining funcs' with struct return types
	let v1 = Vehicle{
		manufacturer: man,
		model: model,
		color: color
	};

	return v1;
}

// Create instances of structs
#[allow( dead_code, unused_variables )]
fn test_structs() {
	let person1 = Person{
		Name: String::from( "John" ), // or simply: '"John".to_string()' (recommended),
		Age: 22
	}; // This is how a we construct struct instances in Rust

	let mut person2 = Person{ // Struct instances are also immutable by default
		Name: "John".to_string(),
		Age: 30
	};
	person2.Name = "Doe".to_string();

	let person3 = Person{
		Name: "Kevin".to_string(),
		Age: 35
	};

	let det_p1 = Person::get_full_details( &person1 );
	let det_p2 = Person::get_full_details( &person2 );
	let det_p3 = Person::get_full_details( &person3 );

	// println! (
	// 	"{:?}\n{:?}\n{:?}", det_p1, det_p2, det_p3
	// );

	// ...
}

// Tuple structs: Used to construct a struct with indexed attributes instead of named attributes.

#[allow( dead_code )]
#[derive( Debug )]
struct PersonTuple( String, u8 ); // This is a tuple struct

#[allow( unused_variables, dead_code )]
fn test_tup_struct() -> () {
	let person_tup: PersonTuple = PersonTuple( "John Doe".to_string() , 36 );

	println! ( "{:?}", person_tup ); // We can access each item of the tuple by supplying the index of the attr after the var name, i.e. 'person_tup.n'
}

/*
Attributes.

- In Rust, attributes are metadata applied to items in our code, such as functions, modules, crates, etc. They effect how the compiler treats them.
- Attributes begin with a '#' symbol and are closed in square brackets, like '#[]'.
- Attributes can also be set to the entire module or crate, like '#![]'.
- Some examples of attributes (although we have used them throughout the code):
	- '#[warn( ... )]' and '#[allow( ... )]': These are called Lint Controls, and they suppress or enable compiler warning.
	- '#[derive( ... )]': Auto-implements traits (simpler 'impl Trait for Struct/Enum').

	- '#[cfg( .. )]': Conditional Compilation. Includes code only if a certain condition is met, like a platform, feature flag, or test mode. E.g. '#[cfg( target_os = "linux"]'.
	- '#[test]': Defines a component as a unit test.

	- '#[doc = ...]': This is called the Documentation Attribute. It is used to attach documentation comments to items like structs, enums, funcs, modules, and crates (equivalent to using triple slashes, but it is more flexible and recommended).
		- Doc attributes can also be styled using markdown.
		- Some Doc sub-attributes:
			- include_str!( path ): embed external documentation.
			- hidden: Hide from documentation.
			- cfg: Helps us use doc attrs as conditional docs, i.e. documentation is visible if the conditional is met. it's alternative is 'cfg_attr'... (I don't recommend using these, makes things complicated lol).
			- ...
*/

/*
Result module.

- Result<T, E> is the type used for returning and propagating errors. It is an enum with the variants Ok(T), representing success and containing a value, and Err(E), representing error and containing an error value.
- Functions return Result whenever errors are expected and recoverable.
- To get the value contained inside the Result we can pass a '.unwrap()' to it.
*/

#[allow( dead_code )]
fn test_Result( a: i8, b: i8 ) -> Result<u8, String> { // The return types can be of any type...
	match b {
		0 => return Err( "Cannot divide with zero".to_string()),
		_ => {
			let res = a / b;

			return Ok( res.try_into().unwrap() ); // ...!
		}
	}
}

#[allow( dead_code, non_snake_case )]
// Better and easier (separate) error handling - let's say the above example doesn't implement the matching, and it is done on the caller's logic instead...
fn test_Result_1( a: i8, b: i8 ) -> Result<u8, String> {
	if b == 0 {
		return Err( "Cannot divide with zero.".to_string() );
	} else {
		let res = a / b;

		return Ok( res.try_into().unwrap() );
	}
}

#[allow( dead_code, non_snake_case )]
fn test_Result_2() -> () {
	let result1 = test_Result_1(22, 0);
	let result2 = test_Result_1(22, 4);

	// Handle the first result
	let x = match result1 {
		Ok( n ) => println! ( "Res1 is: {:?}", n ),
		Err( e ) => println! ( "Error1: {:?}", e )
	}; // This method is called a 'let match binding'
	let y = match result2 {
		Ok( n ) => println! ( "Res2 is: {:?}", n ),
		Err( e ) => println! ( "Error2: {:?}", e )
	};
}

// Easier error handling can be done with the question mark operator '?', making error handling much cleaner, without having to write verbose match and if let blocks

/*
- There is also a different type of error handling, it is the std::io::Result.

*/

// ...

// Conditional lets...
#[allow( dead_code )]
fn let_conditionals() {
	// if let

	// while let
}

pub mod advanced_concepts;

pub mod smart_pointers;
pub mod threading;

// Rust requires a main entry function!
fn main() {
	// vars_and_data_types();
	// functions_and_modules();
	// test_if();
	// test_while();
	// test_loop();
	// test_for();
	// closures::test_closures();
	// closures::test_closures_2();
	// closures::test_closures_3();
	// test_match_int();
	// test_op_shapes_check();
	// test_direct_impl();
	// test_trait_impl();
	// test_structs();
	// test_tup_struct();
	// helpers::a_doc_fn();
	// helpers::fn_for_Linux();
	// let res = test_Result(22, 4);
	// println! ( "Res is: {:?}", res.unwrap() );
	// test_Result_2();

	// advanced_concepts::test_mut_structs();
	// advanced_concepts::test_trait_bound_fn();
	// advanced_concepts::test_ownership();
	// advanced_concepts::test_clone_trait();
	// advanced_concepts::test_vec_int();
	// advanced_concepts::test_vec_str();
	// advanced_concepts::test_vec_custom();
	// ...
	let log = advanced_concepts::test_dec_macros();

	println! ( "{log}" );

	// ...
}
