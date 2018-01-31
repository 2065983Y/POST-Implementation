// Trait for listeners to implement
// TODO: refactor
//	-- DONE 1. Abstract message related methods to Carrier

extern crate iron;

use local;


pub trait Listener {
	fn listen(&self, local: local::Local);

}

