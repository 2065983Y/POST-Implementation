
use message::Message;

pub trait ISendable<T> {
	type Item;

	fn encode(&self) -> T;

}
