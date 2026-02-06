/*

Exercises for Rust [02]
	01. Funcitons with string arg and Option type returns (str for now, with conversion)
    02. Functions with custom enum type returns
    03. Functions with an Option type return and a custom argument type

	04. Enums and function implementations for them (do w/ shape)
	05. Enums with traits (area trait)

	06. Structs and function implementations for them (do w/ person)
	07. Structs with traits (check_age trait and display_info raw impl)

	08. Importing enums, traits, and structs from modules
*/

#[allow( dead_code, unused_assignments, non_snake_case )]
fn check_option_1( Name: String ) -> Option<String> {
	let mut this_str: Option<String> = None; // Init the var as a null object
	this_str = Some( String::from( Name ) );

	return this_str;
}

#[allow( dead_code )] // Used here because it throws a 'field 0 is never read' error
#[derive( Debug )]
enum ReturnType{
	AMessage( String ),
	AnError( String ),
}

#[allow( dead_code, unused_assignments, non_snake_case )]
fn check_custom( command: &str ) -> ReturnType {
	// Simple match case
	match command {
		"hello" => return ReturnType::AMessage( String::from( "This is a Message!" ) ),
		"world" => return ReturnType::AnError( String::from( "This is an Error!" ) ),
		&_ => return ReturnType::AnError( String::from( "This is also an Error!" ) ) // This is the callback for string literal pattern matching. It is required!
	}
}

fn check_option_2( message: ReturnType ) -> Option<ReturnType> {
	match message {
		// Simple argument parsing with match
		ReturnType::AMessage( r ) => return Some( ReturnType::AMessage( r ) ), // The r here is just a variable bound to the string inside the enum (pattern binding/matching)
		ReturnType::AnError( r ) => return Some( ReturnType::AnError( r ) ),
	}
} // Rust's pattern matching is very powerful

#[allow( dead_code )]
#[derive( Debug )]
// We create a separate enum to be used as a return type
enum Return { // They match the Shape objects args!
	CircleArea( f64 ),
	RectArea( u8 )
}

#[allow( dead_code, non_snake_case )]
enum Shape {
	Circle( f64 ),
	Rect( u8, u8 ) // We went with an int instead
}

// Raw implementation
#[allow( dead_code, unused_assignments )]
impl Shape {
	fn return_something( &self ) -> &str { return "Hi there!" } // Lol
}

#[allow( dead_code )]
trait Area {
	fn area( &self ) -> Return; // The return type enum is used for this trait
}

#[allow( dead_code )]
impl Area for Shape {
	fn area( &self ) -> Return {
		match self {
			Shape::Circle( r ) => return Return::CircleArea( std::f64::consts::PI * r * r ),
			Shape::Rect( w, h ) => return Return::RectArea( w * h )
		}
	}
}

trait PersonT {
	fn check_age( &self ) -> String;
}

#[allow( non_snake_case )]
struct Person {
	Name: String,
	Age: u8
}

#[allow( unused_assignments )]
impl Person {
	fn display_info( &self ) -> String {
		let mut info: String = String::new();

		// String concatenation with some formatting
		info = format! ( "Person {0} is {1} years old!", self.Name, self.Age );
		return info;
	}
}

impl PersonT for Person {
	fn check_age( &self ) -> String {
		match self.Age { // Match it to the struct's Age attribute
			1..15 => return String::from( "Person cannot drive!" ), // 1 to 14 cannot drive
			15 | 16 => return String::from( "Person is almost ready to drive!" ), // 15 or 16 is an almost
			16..80 => return String::from( "Person can drive!" ), // 16 to 79 can driver
			_ => return String::from( "Person is prolly dead!" ) // Rest is R.I.P
		}
	}
}

pub mod module2;

#[allow( dead_code, non_snake_case )]
fn main() {
	let ch = check_option_1( String::from( "Aaron" ) );
	println! ( "Ex 01\nName is {}!\n", ch.unwrap() );

	let ch2 = check_custom( "hello" );
	println! ( "Ex 02\n{:?}\n", ch2 );

	let ch3 = check_option_2( ReturnType::AMessage( String::from( "Hello World!" ) ) );
	let ch3_2 = check_option_2( ReturnType::AnError( String::from( "Hello World, Error 404!" ) ) );
	println! ( "Ex 03\n{:?}\n{:?}\n", ch3, ch3_2 );

	let Circle = Shape::Circle( 2.4 );
	let Rect = Shape::Rect( 4, 3 );

	println! ( "Ex 04 and 05\nArea of circle: {:?}\nArea if rect: {:?}\n", Circle.area(), Rect.area() );

	let person1 = Person{
		Name: String::from("John Doe"),
		Age: 16
	};
	println! ( "Ex 06 and 07\n{}\n{}\n", person1.check_age(), person1.display_info() );

	// Rust's compiler does everything to help us correct our errors, big W!!!

	let animal1 = module2::Animal{
		Name: String::from( "Dog" ),
		Barks: true
	};
	println! ( "Ex 08\n{:?}\nDoes the Animal Bark?: {:?}", animal1, module2::AnimalTrait::does_animal_bark( &animal1 ) );
}
