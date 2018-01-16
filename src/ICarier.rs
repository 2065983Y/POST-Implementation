// Trait for carriers to implement
// TODO: needs refactoring
//	1. Add Message events in carrier

use Remote;

pub trait ICarier {

	fn init(&self, remote: Remote) -> ICarier;
}
