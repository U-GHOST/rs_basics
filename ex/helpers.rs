// Ex 3 module

#[allow( non_snake_case, dead_code, unused_assignments )] // This is global to all types
enum ReturnTypes {
	AMessage( String ),
	AnError( String )
}

impl ReturnTypes {
	// Add implementations!
}

// Better structuring of the codebase
#[allow( non_snake_case, dead_code, unused_assignments )]
pub mod PersonModule {
	pub trait PersonTraits {
		fn check_age( &self ) -> bool; // Checks the person's age and returns a bool (or something else)*
		fn display_info( &self ) -> String; // Constructs a string from the struct's attributes and returns a string
	}

	#[derive( Debug )]
	pub struct Person {
		pub Name: String,
		pub Age: u8,
		pub Email: String,
		pub DoB: String,
	}

	impl PersonTraits for Person {
		fn check_age( &self ) -> bool {
			let PersonAge: u8 = self.Age;

			match PersonAge {
				..=16 => return false, // From 0 to 16
				17..=79 => return true, // From 17 to 79
				_ => return false // The rest above
			}
		}

		fn display_info( &self ) -> String {
			let PersonName: &String = &self.Name;
			let PersonAge: &u8 = &self.Age;

			let PersonString: String = format! ( "Person {0} is {1} years old!", PersonName, PersonAge );

			return PersonString;
		}
	}
}

#[allow( non_snake_case, dead_code, unused_assignments )]
pub mod VehicleModule {
	#[derive( Debug )]
	pub enum VehicleMan {
		Toyota,
		Mercedes,
		Volks
	}

	#[derive( Debug )]
	pub enum VehicleColor {
		Red,
		Green,
		Blue,
		Black,
		White,
		Gray
	}

	pub trait VehicleTraits {
		fn get_man_date( &self ) -> String;
	}

	#[derive( Debug )]
	pub struct Vehicle {
		pub Brand: VehicleMan,
		pub Model: String,
		pub Color: VehicleColor,
		pub ManDate: String,
	}

	impl VehicleTraits for Vehicle {
		fn get_man_date( &self ) -> String {
			let man_date: &String = &self.ManDate;

			return man_date.to_string();
		}
	}
}

pub enum Direction {
	North,
	South,
	East,
	West
}

#[allow( non_snake_case, dead_code, unused_assignments )]
impl Direction {
	fn is_vertical( &self ) -> String {
		match self {
			Direction::North | Direction::South => "Direction is Vertical!".to_string(),
			_ => "Direction is not Vertical!".to_string()
		}
	}
}
