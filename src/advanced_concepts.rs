/*
Advanced concepts in Rust. Expanded definitions...
*/

/*
Iterator mapping and collection.

- ...
*/
#[allow( dead_code, non_snake_case )]
fn test_iter_map_coll() {
	// ...
}

/*
Generics.

- Generics allow us to write code that works with different data types without repeating the same logic for each one.
- They are useful for avoiding repitition, type safety, and flexibility.
*/

#[allow( dead_code, unused_variables )]
fn generic_func<T>( value: T ) { // The 'T' is called a generic type placeholder, and it can be letter or word and must represent a trait!
	// We can use 'value', no matter what type it is, or map it to an enum or struct or trait (implemented on structs below...covered the others in exercises)
}

// Generic functions using trait bounds (used to specify constraints on generic types. They allow us to define what traits a type must implement in order to be used with a particular func)
#[allow( dead_code )]
trait Animal {}

#[allow( dead_code )]
struct Dog;

#[allow( dead_code, non_snake_case )]
#[derive( Debug )]
struct Cat {
	Name: String,
	Age: i16
}

impl Animal for Cat {}

#[allow( unused_variables )]
fn apply_trait_bounds<T: Animal>( animal: T ) {
    // Now, when we call this generic function, we must supply an argument that implements the Animal trait (Animal is called a generic type argument to the generic type placeholder, together called generic type declaration)
	// animal.drink();
}

#[allow( dead_code, unused_variables )]
pub fn test_trait_bound_fn() {
	let dog: Dog = Dog;
	let cat: Cat = Cat{
		Name: "Mittens".into(),
		Age: 5
	};

	// let t_dog = apply_trait_bounds( dog ); // This will not work, since the struct Dog doesn't implement the trait Animal
	let t_cat = apply_trait_bounds( cat );

	// println! ( "{:?}", cat );
	/*
	- The above print on the 'cat' instance will cause a panic. Rust's ownership model doesn't let us use the instance's value after the value is moved, i.e. 't_cat' owns it.
	- We need to implement a 'Copy' or 'Clone' trait on the cat var and copy/clone the value for safety (examples for both below). We can also use Smart Pointers, which will be covered later.
	*/
}

// Clone implemented
#[allow( dead_code, non_snake_case )]
#[derive( Clone, Debug )]
struct BCat {
	// Name: String, // Here, the compiler will throw an error.
	Name: String,
	Age: u8
}

/*
- Strings cannot implement the 'Copy' trait. Because of Rust's ownership model, if a String implemented a Copy trait, it could lead to situations where two vars own the same heap. When one var goes out of scope, it would free the mem, leaving the other var with a dangling var. They can implement the 'Clone' trait.
*/

impl Animal for BCat {}

#[allow( non_snake_case )]
pub fn test_clone_trait() -> () {
	let bCat: BCat = BCat { Name: "Cat".into(), Age: 22 };

	// println! ( "{:?}", bCat );
	let t_bCat = apply_trait_bounds( bCat );

	println! ( "{:?}", t_bCat )
}

/*
A Quick Summary:

- Use &var for immutable references.
- Use &mut var for mutable references.
- Use var.to_owned() to create an owned version of borrowed data.
- Use var.copy() for types that implement the Copy trait (like integers).
- Use var.clone() for a deep copy of types that implement the Clone trait.
*/

// Generic structs
#[allow( non_snake_case, dead_code )]
struct PersonX<T> { // The 'T' generic placeholder's argument type (g t arg) can be defined when the struct is constructed (for example, when creating an instance, etc.)
	Name: String,
	Age: T
}

// Usage: 'let person: PersonX<u8> = Person{};'. This makes the 'T' placeholder of type 'u8', and the Age aatribute a u8

#[allow( non_snake_case, dead_code )]
struct Person<PetType: Animal> { // Now, when an instance is constructed, the Pet attribute must have an Animal trait (as a generic type arg) as its value
	Name: String,
	Pet: PetType
}

#[allow( unused_variables )]
pub fn test_gen_structs() {
	let cat: Cat = Cat{
		Name: "Mittens".into(),
		Age: 5
	};

	let person = Person{
		Name: "John Doe".to_string(),
		Pet: cat
	};
}

/*
Combining traits.

- Generic types can combine traits using '+'.
- Let us remake the animals example with a new trait and struct.
*/

#[allow( dead_code )]
trait NotDangerous {}
#[allow( dead_code )]
struct Bear;

impl NotDangerous for Dog {}
impl NotDangerous for Cat {}

// Here is a struct that combines two traits
#[allow( dead_code, non_snake_case )]
struct StrictPerson<PetType: Animal + NotDangerous> { // This struct requires the Pet attr to impl both traits to be assigned/take value correctly, i.e. Cat only
	Name: String,
	Pet: PetType
}

// Another way to define generic types... (mostly used for multiple generic types)
#[allow( dead_code, non_snake_case )]
trait SomeTrait {}
#[allow( dead_code, non_snake_case )]
struct SomeStruct;
#[allow( dead_code, non_snake_case )]
struct StricterPerson<PetType, StructType>
where
	PetType: Animal + NotDangerous,
	StructType: SomeTrait
{
	Name: String,
	Pet: PetType,
	Struct: StructType
}

/*
Covering Rust's std traits.

Copy
Clone
Sized
...
*/

/*
Common Collections.

- Rust's common collections can be grouped into four major categories:
	- Sequences: Vec, VecDeque, LinkedList
	- Maps: HashMaps, BTreeMap
	- Sets: HashSets, BTreeSet
	- Misc: BinaryHeap

A. Vectors *Extended*.
- Vectors are a collection type that let us store lists of values in a single data structure.
- They can only store values of the same type ('Vec<T>' of type T).
- We can define a vector using its typed name (Vec<T>) or macro (vec![]).
- Vector operations can be performed on mutable vectors, notably the 'push' and 'pop', to add values to the end of a vector, or pop values (elaborated more in examples).
- We can also easily iterate over vectors (shown in examples).

B. HashMaps (Dicts from Python).
- ...

C. HashSets
- ...
*/

#[allow( dead_code, unused_variables )]
pub fn test_vec_int() {
	let int_vec = vec![1, 2, 3]; // This is a vec (immutable)
	let int_vec_mut = vec![3, 2, 1]; // This is also a vec (mutable)

	let mut int_vec_2: Vec<i32> = Vec::new(); // Creates an empty vec (vectors defined w/ typed names must specify the types in the vec, i.e. 'x: Vec<T> = Vec::new()')
	int_vec_2.push( 0 );
	int_vec_2.push( 30 );

	int_vec_2.pop(); // Removes the last element, i.e. '30'

	println! ( "{:?}", int_vec_2 );

	println! ( "Vec size: {:?}", int_vec_2.len() );
	println! ( "Capacity size: {:?}", int_vec_2.capacity() ); // Since vectors are dynamically resizable, their capacity increases dynamically

	// Accessing individual elements
	println! ( "The first item is: {:?}", int_vec_2[0] ); // Ranges aren't supported here, unless...

	// Changing ownership of vectors using Vec
	let int_vec_2_new: Vec<i32> = Vec::from( int_vec_2 ); // Basically 'int_vec_2_new = int_vec_2', but fancier
	println! ( "New owner has: {:?}", int_vec_2_new ); // int_vec_2_new now owns the values of int_vec_2

	let mut int_vec_3: Vec<i32> = Vec::new();
	int_vec_3.push( 47 );
	int_vec_3.push( 33 );
	int_vec_3.push( 45 );
	int_vec_3.push( 2 );
	int_vec_3.push( 0 );
	println! ( "Elements in range: {:?}", &( int_vec_3 ).as_slice()[0..=4] ); // We need grab a reference to the vector

	// ...
}

#[allow( dead_code, unused_variables )]
pub fn test_vec_str() -> () {
	// Iterating over vectors
	let first_names = vec![
		"John",
		"Wick",
		"Trevor",
		"Doe",
		"Elliot"
	];

	for first_name in &first_names { // Referenced to it for reuse (iterating also changes ownerships in Rust)
		// Or we can also do 'for x in y.as_slice() or .clone() {...}'
		println!( "Name is: {:?}", first_name );
	}

	// Iteration with enumeration
	for ( x, f_name) in first_names.iter().enumerate() {
		println! ( "Index: {:?}, Name: {:?}", x, f_name );
	}
}

#[allow( dead_code )]
#[derive( Debug )]
struct Car {
	name: String,
	model: String
}

// Vectors can also contain custom types (like structs, enums, etc...)
#[allow( dead_code, unused_variables )]
pub fn test_vec_custom() -> () {
	let mut car_lot1: Vec<Car> = Vec::new();
	let mut car_lot2: Vec<Car> = Vec::new();
	let mut car_lot3: Vec<Car> = Vec::new();

	// Using for with ranges
	for _ in 1..=5u8 { // Pushes it 100 times
		car_lot1.push( Car{ name: "Porsche".to_string(), model: "Panamera".to_string() });
	}

	for _ in 1..=5u8 { // Pushes it 100 times
		car_lot2.push( Car{ name: "Hyundai".to_string(), model: "Sanata".to_string() });
	}

	println! ( "Car lot 1 has: {:?}\n", car_lot1 );
	println! ( "Car lot 2 has: {:?}\n", car_lot2 );

	// Appending: Moves values from a vector to another: 'vecX.append( &mut vecY );'
	car_lot3.append( &mut car_lot1 ); // Now car_lot1 is empty

	// We can also insert new elements into the merged vector by specifying the index
	car_lot3.insert( 0, Car{
		name: "Lamborghini".to_string(),
		model: "Aventador".to_string()
	}); // This will shuffle everything accordingly

	println! ( "Car lot 1 has: {:?}\n", car_lot1 );
	println! ( "Car lot 2 has: {:?}\n", car_lot2 );
	println! ( "Car lot 3 has: {:?}\n", car_lot3 );

	// Removing elements from the merged vector
	// car_lot3.remove(0);

	// Retains only the elements that meet the conditionals
	car_lot3.retain( |e: &Car| { if e.name == "Lamborghini" { return true; } else { return false; } } );

	println! ( "Car lot 3 has: {:?}\n", car_lot3 );

	// Pre-allocating mem to our vec
	car_lot3.reserve( 50 ); // Reserves an extra 50 beyond the current len of the vec
}

/*
Linting.
*/

// Move semantics

// Lifetimes

/*
Macros.
- Macros in Rust are a meta-programming concept that help us generate extra code during compile-time.
- They help keep things simpler by reducing repititive code writing.
- There are two types of Macros in Rust:

A. Declarative Macros
- Look like a function, declared with an exclamation mark, and easy to write.
- Some common examples:
	- println!()
	- panic!()
	- format!()
	- stringify!()
	- vec![]
	- env!()
	- include!()
- Unlike regular functions, declarative macros are not evaluated on runtime.
- Dec macros can have their own syntax, like using a '()' or a '{}'. They enable users to extend the Rust syntax.
- Dec macros are defined using the 'macro_rules!' keyword. Check example below...

B. Procedural Macros
- ...
*/

// Defining the macro
macro_rules! log { // A simple logging macro
	( $msg:literal ) => {
		concat! ( "# ", $msg )
	};
}

/*
- In this dec macros definition, '$msg:literal' specifies the pattern the macro will match.
- The '$msg' is a metavariable that will capture whatever literal value you pass to the macro.
- The ':literal' specifies that the captures value must be a literal (e.g. a string, int, bool, etc.). They are called 'Fragment Specifiers'.

- Using this macro will be the equivalent to (comparison):

let log = log! ( "Hello World" ); -> let log = concat! ( "# ", "Hello World" );
*/

pub fn test_dec_macros() -> &'static str {
	return log! ( "Hello World" );
}

// File operations
// ...

// Environment variables
// ...

// Exec Arguments
// ...

// 

// Unsafe types*
// ...
