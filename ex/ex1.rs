/*

Exercises for Rust [01]
	01. Conversion of primitive data types (f64 to u8)
	02. Calling functions of a calc (for u16 only!)
	03. Getting an int input and running if conditionals
	04. Iterating through an array
	05. Getting a string input, parsing to int, and running while loops

	06. Running closures from a function
	07. Running closures with parameters from a function
	08. Importing modules and running functions
*/

fn conv_p( a: u16, b: f64 ) -> u16 {
	let ret = a + b as u16;

	return ret;
}

#[allow( dead_code )]
fn calc_sum( a: u16, b: u16 ) -> u16 {
	let sum: u16 = a + b;

	return sum;
}

fn calc_diff( a: u16, b: u16 ) -> u16 {
	let diff: u16 = a - b;

	return diff;
}

fn calc_prod(  a: u16, b: u16 ) -> u16 {
	let diff: u16 = a * b;

	return diff;
}

fn calc_res(  a: u16, b: u16 ) -> u16 {
	let diff: u16 = a / b;

	return diff;
}

fn test_if_int( input: i16 ) {
	if input > 16 {
		println! ( "User can drive a car!\n" );
	} else if input == 16 {
		println! ( "User is close to driving a car!\n" );
	} else {
		println! ( "User cannot drive a car!\n" );
	}
}

fn iterate_arr( arr: [u16; 5] ) {
	for x in arr {
		if x > 20 {
			println! ( "Above 20" );
		} else {
			println! ( "Below 20" );
		}
	}
}

fn test_while( mut num: u16 ) {
	while num < 16 {
		println! ( "Still Iterating..." );

		num += 1;
	}
}

fn closure_test() {
	let clos = || {
		println! ( "Hello from the closure!" );
	};

	clos();
}

fn closure_test_2( a: &str, b: &str ) {
	let clos = |x: &str, y: &str| {
		println! ( "Hello World, {} {}!", x, y );
	};

	clos( a, b );
}

pub mod module1;

fn main() {
	println! ( "Ex 01:" );
	let ret = conv_p( 20, 35.5 );
	println! ( "{}\n", ret );

	println! ( "Ex 02.1:" );
	let sum = calc_sum( 23, 24 );
	println! ( "Sum is {}", sum );

	println! ( "Ex 02.2:" );
	let diff = calc_diff( 47, 40 );
	println! ( "Diff is {}", diff );

	println! ( "Ex 02.3:" );
	let prod = calc_prod( 100, 5 );
	println! ( "Diff is {}", prod );

	println! ( "Ex 02.4:" );
	let res = calc_res( 47, 47 );
	println! ( "Diff is {}\n", res );

	// Take user input
	println! ( "Ex 3.0\nEnter your age:" );
	let age = &mut String::new();
	std::io::stdin().read_line( age ).unwrap();
	// Convert to i16
	let age_c = age.replace( "\n", "" ).parse::<i16>().unwrap();

	test_if_int( age_c );

	println! ( "Ex 4.0" );
	let arr = [20, 32, 16, 50, 92];

	let it = iterate_arr( arr );

	println! ( "{:?}\n", it );

	println! ( "Ex 5.0" );
	let num: u16 = 0;
	test_while( num ); println! ();

	println! ( "Ex 6.0" );
	closure_test();
	println! ( "Ex 7.0" );
	closure_test_2( "Hello", "Rust" ); println! ();

	println! ( "Ex 8.0" );
	module::helper();
	println! ( "Ex 9.0" );
	module::child_helper::sub_helper();
}
