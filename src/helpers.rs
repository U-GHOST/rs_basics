// This is a simple module

// Functions (or variables*) inside a module are private by default, and aren't visible to the caller script. Therefore, we need to declare them as public using 'pub fn' for visibilty
pub fn get_name_mod( first_name: &str, last_name: &str ) -> String {
	let full_name = format! ( "{} {}", first_name, last_name );

	return full_name;
}

// Creating a child module inside a module (also private by default, if not declared)
pub mod namehelpers {
	fn insider() { // This is a private function inside the child module
		println! ( "\nHello from private child module function!\n" );
	}

	pub fn get_name_child_mod( first_name: &str, last_name: &str ) -> String {
		let full_name = format! ( "{} {}", first_name, last_name );

		insider(); // The private function can be called inside the child module itself

		return full_name;
	}
}

pub mod agehelpers {
	pub fn add_5_to_age( age: u16 ) -> u16 { // A public child module function that takes a u16 as input, adds 5, and returns a u16

		return age + 5;
	}
}

#[doc = "
This is a simple function to show docs!

Docs can be single-line or multi-line.

They are showed exactly how you write them...
"]
pub fn a_doc_fn() -> () {
	println! ( "This is a simple function to show docs!" )
}

#[allow( non_snake_case )]
#[cfg( target_os = "linux" )]
pub fn fn_for_Linux() {
	println! ( "This instructs the compiler to compile this function for Linux only!" );
}

#[allow( non_snake_case )]
#[cfg( target_os = "windows" )]
pub fn fn_for_Windows() {
	println! ( "This instructs the compiler to compile this function for Windows only!" ); // It cannot be viewed or accessed in other OSes
}
