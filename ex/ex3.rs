/*

Exercises for Rust [03]
	00. Write a full Rust program that defines enums and structs with traits, calls them from a child-module* inside a module and constructs instances -> Designed w/ Pseudo-code.
*/

pub mod helpers;

#[allow( unused_imports )]
use helpers::{
	PersonModule::{
		Person,
		PersonTraits,
	},

	VehicleModule::{
		Vehicle,
		VehicleColor,
		VehicleMan,
		VehicleTraits
	}
};

fn with_sp_use() {
	let person1 = Person {
		Name: "John Doe".to_string(),
		Age: 22,
		Email: "johndoe@rust.com".to_string(),
		DoB: "August 03 2003".to_string()
	};
	let age_check = PersonTraits::check_age( &person1 );

	println! ( "{:?}\nCan person drive? {:?}", person1, age_check );	
}

fn main() {
	with_sp_use();
}
