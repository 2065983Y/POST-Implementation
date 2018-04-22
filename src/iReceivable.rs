
pub trait IReceivable<T> {

	fn decode(&mut self) -> Option<T>;
}
