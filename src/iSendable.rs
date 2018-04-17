
pub trait ISendable<T> {

	fn encode(&self) -> T;

}
