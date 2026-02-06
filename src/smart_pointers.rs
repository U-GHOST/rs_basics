#![allow(
	unused_assignments,
	unused_imports,
	dead_code,
	non_camel_case_types,
)]

use std::cell::{Cell, RefCell}; // Must be imported for interior mutability
use std::rc::Rc;

/*
Smart Pointers.
- A Pointer ...
- Smart pointers in Rust are data structures that act like regular pointers but provide additional functionality and metadata, primarily for managing memory and ownership safely and efficiently.
- How does Rust's memory management differ from other languages:
	- In C: Manual memory management, which is very unsafe! (malloc/free).
	- In C++: Manual + RAII (Resource Acquisition Is Initialization) + smart pointers.
	- Java and Python: Garbage Collector.
- Rust has no garbage collector, and it has deterministic, compile-time mem safety. And the ownership-borrowing model helps it create fast programs with mem safety.
*/

// Memory architecture and Rust

/*
Stack:
- Fast.
- Fixed size.
- Last-in-First-out.
- Automatically cleaned up.
*/

pub fn foo_stack() {
	// These variables are stored in the stack.
	// Rust uses the stack if it knows the size of the variable at compile-time.
	let x = 10;
	let y = true;

	// When foo_stack() exits, x and y are gone.
	// Rust guarantees no dangling references in the stack (unlike C++).
}

/*
Heap:
- Slower than stack.
- Dynamically sized.
- Explicit allocation.
- Freed automatically by ownership rules.
*/

pub fn foo_heap() {
	// A String is composed of its pointer + length + capacity + actual string data.
	// The p (points to the var in mem) + l (counts the chars in the var) + c (reserves extra space for the var in mem when it needs space to grow) are stored in the stack actually.
	// The actual string data is stored in the heap.
	let s = String::from( "Hello" );
	let s1 = "World"; // String literals only have a p + l + actual data, but the actuall data is not stored on the stack or the heap (different and complicated type tbh).
}

// Box<T>: "Heap allocation with single ownership". It allocates a value on the heap, has single ownership, and no runtime cost beyond allocation.
// Why use Box?:
	// Large data we don't want on the stack
	// Recursive types
	// Trait objects

// Box for recursive types
struct ListNode<T> {
	value: T,
	// We use Box for recursive typing where a type recursively uses itself...
	prev: Option<Box<ListNode<T>>>,
	next: Option<Box<ListNode<T>>>,
}

struct LinkedList<T> {
	head: Option<Box<ListNode<T>>>,
	tail: Option<Box<ListNode<T>>>,
} // We used an Option<T> because LinkedList's head.prev and tail.next are None.

// Box for traits (mostly a costly alternative to enums)
trait Animal { // A simple example
	fn mammal( &self ) -> bool;
}

// Let's say we want the input to be a struct that implements the trait Animal as a struct's field, but traits cannot be used directly (we don't know the size of the structs implementing it)
// This is very basic for functions
fn check_animal<A: Animal>( animal: A ) {
	if animal.mammal() {
		println! ( "This Animal is a mammal!" );
	} else { println! ( "This Animal is not a mammal!" ) }
}

struct Dog;

impl Animal for Dog {
	fn mammal( &self ) -> bool { true }
}

struct Cat;

impl Animal for Cat {
	fn mammal( &self ) -> bool { true }
}

// But for fields...
struct LivingThingA {
	lt_type: Box<dyn Animal>, // And we need the dyn keyword (which is a prefix of a trait object's type, has tradeoffs like additional runtime cost)
} // We can then assign its value with Box::new( T )

// Generics can be used to directly store the type in the stack.
// This is considered safe, and needs the type declared when defining the variable.
struct LivingThingB<A: Animal> {
	lt_type: A
}

fn x() {
	let lt_B: LivingThingB<Cat> = LivingThingB { lt_type: Cat };
}

// Cell: A Cell is a simple Smart Pointer that allows for interior mutability on immutable variables.
#[derive( Debug )]
struct  MutPerson<'p> {
	Name: String,
	Something: Cell<&'p str>, // Strings cannot be wrapped inside cells, so we use a string slice with a lifetime specifier (must match the specifier used for the struct)
	Age: Cell<u16>
}

pub fn test_mut_structs() {
	let mut_person: MutPerson<'_> = MutPerson{ // !!
		Name: "John Doe".to_string(),
		Something: Cell::from( "This is just something" ),
		Age: Cell::from( 47 )
	};

	/*
	- Here, reassigning a value with 'Cell::from( "..." )' won't work, and will throw an 'immutable variable error'. But we don't want the whole variable to be mutable.
	- Solution: Use '.set( "..." )'
	*/

	// mut_person.Something = Cell::from( "This also something" ); // This won't work
	mut_person.Something.set( "This is also another something" );

	println! ( "{:?}", mut_person.Something.get() ); // We can unwrap the value of the Cell<T> by supplying '.get()'
}

// RefCell<T>
// Skipped this, not that relevant for me

// Rc<T>: Rc, of Reference Counting, is a Smart Pointer that enables multiple ownership of data. It keeps track of the number of refs to the data it points to, allowing us to have multiple owners of the same data while ensuring that the data stays alive as long as there are references to it.
// They are immutable by default (we can use it with Rc<RefCell<T>> for interior mutability). The data inside Rc<T> is stored in the heap, the Rc itself is stored on the stack.
// Each time an Rc is cloned, the reference count is increased. When an Rc is dropped, the count is decremented. When the count reaches zero, the data is deallocated.
struct Node {
	value: i32,
	left: Option<Rc<Node>>,
	right: Option<Rc<Node>>,
}

fn check_rc() {
	let node_a = Rc::new( Node { value: 47, left: None, right: None } );
	let node_b = Rc::new( Node { value: 48, left: None, right: None } );

	// Share nodes a and b in a parent node
	let parent = Node {
		value: 3,
		left: Some( Rc::clone( &node_a ) ),
		right: Some( Rc::clone( &node_b ) )
	};

	// Each variable lives
	println! ( "Parent: {}!", parent.value );
	println! ( "Node A/Parent: {}!", parent.left.unwrap().value ); // !
	println! ( "Node A: {}!", node_a.value );
	println! ( "Node B: {}!", node_b.value );
}
