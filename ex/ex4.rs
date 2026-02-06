// Rust Generics ex

// Implementations cause some mess when using lifetime specifiers with generic types!!

#![ allow( dead_code, unused_assignments, unused_variables, unused_imports, non_snake_case ) ] // Let's just get it over with globally

use std::cell::Cell;

trait Animal {} // Add methods!
trait NotDangerous {}

struct Cat<'C> {
	Name: Cell<&'C str>,
	Age: Cell<&'C u8>
}
impl Animal for Cat<'_> {}
impl NotDangerous for Cat<'_> {}

struct Dog<'D> {
	Name: Cell<&'D str>,
	Age: Cell<&'D u8>
}
impl Animal for Dog<'_> {}
impl NotDangerous for Dog<'_> {}

struct Bear {}
impl Animal for Bear {}

trait JobTypesTrait {}

enum JobTypesEnum<> {
	Accountant,
	Carpenter,
	Mechanic
}

impl JobTypesTrait for JobTypesEnum<> {}

enum BirthMonth<> { // Used as a struct attr type (to compare ease of use between generic types and enums!)
	Jan,
	Aug,
	Nov
}

struct Person<'P, JobType, PetType>
where
	JobType: JobTypesTrait,
	PetType: Animal + NotDangerous
{
	Name: Cell<&'P str>,
	BM: BirthMonth, // !
	Job: Cell<&'P JobType>,
	Pet: Cell<&'P PetType>
}

// Generic types using trait bounds
fn accept_gen_args<T: Animal>( obj: T ) { // Argument 'obj' must implement the Animal trait...
	// ...
}

fn main() -> () {
	let Cat1: Cat<'_> = Cat{
		Name: Cell::from( "Gunther" ),
		Age: Cell::from( &6 )
	};

	let Person1 = Person{
		Name: Cell::from( "Simon" ),
		BM: BirthMonth::Aug,
		Job: Cell::from( &JobTypesEnum::Accountant ),
		Pet: Cell::from( &Cat1 )
	};
}
