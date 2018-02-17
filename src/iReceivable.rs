
use message::Message;

pub trait IReceivable<T> {

	fn decode(&mut self) -> T;
}
