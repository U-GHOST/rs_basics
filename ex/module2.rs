// Ex 2 module

#[allow( dead_code, unused_assignments, unused_variables, unreachable_patterns )]
pub enum AnimalEnums { // Identical variants
	Dog,
	Cat,
	Cow,
	Pigeon
}

#[allow( dead_code, unused_assignments, unused_variables, unreachable_patterns )]
pub trait AnimalTrait { // Internal trait
	fn does_animal_bark( &self ) -> bool;
	fn what_is_it( &self, animal: AnimalEnums ) -> String; // Take the enum and returns a string
}
/*
- Functions of traits cannot be used from the struct it is implemented on when called from another sources. Hence, the Animal instance created in the source script cannot call the funcs inside this trait.
- Therefore, the underlying func need to be called with 'AnimalTrait::func' not the struct implementing them.
*/

#[allow( non_snake_case )]
#[derive( Debug )]
pub struct Animal { // Remember, attributes must be set to public for visibility
	pub Name: String,
	pub Barks: bool
}

#[allow( dead_code, unused_assignments, unused_variables, unreachable_patterns )]
impl AnimalTrait for Animal {
	fn does_animal_bark( &self ) -> bool {
		let animal_name = &self.Name;
		let dog: String = String::from( "Dog" ); // Rust makes it very repetetive and boring for safety ('Boring is always best')

		match animal_name {
			dog => return true, // This has a 'unreachable pattern' error. Don't know what it is, just suppressed it
			_ => return false
		}
	}

	// Just a stupid way to exercise
	fn what_is_it( &self, animal: AnimalEnums ) -> String {
		match animal {
			AnimalEnums::Dog => return String::from( "It's is a Dog!" ),
			AnimalEnums::Cat => return String::from( "It's is a Cat!" ),
			AnimalEnums::Cow => return String::from( "It's is a Cow!" ),
			AnimalEnums::Pigeon => return String::from( "It's is a Pigeon!" ),
		}
	}
}
