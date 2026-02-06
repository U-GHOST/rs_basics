// Closures (anonymous functions/lambdas)

pub fn test_closures() {
	let ret_name = || println! ( "Hello Rust!" ); // This marks the variable as a closure function

	// Must be called (or returned) as a function, assigned to a variable, or even called by another closure
	ret_name(); // or 'return add();'
}

pub fn test_closures_2() { // A closure with an input argument
	let ret_name = |x: &str, y: &str| println! ( "{} {}, Hello Rust!", x, y );

	ret_name( "Hello", "World" );
}

pub fn test_closures_3() { // A multi-line closure
	let ret_name = |x: &str, y: &str| {
		println! ( "{} {}, Hello Rust!", x, y );
	};

	ret_name( "Hello", "World" );
}
