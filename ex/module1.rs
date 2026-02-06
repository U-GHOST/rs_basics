// Ex 1 module

pub fn helper() {
	println! ( "This is called from a module helper!\n" );
}

pub mod child_helper {
	pub fn sub_helper() {
		println! ( "This is called from a module's child module!" );
	}
}
