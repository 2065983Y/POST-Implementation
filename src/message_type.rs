use message::Message;

#[derive(Serialize, Deserialize, Debug)]
pub enum MessageType<T> {
	Message(T),
	PartialMessage
}

impl <T> MessageType<T> {

	pub fn unmask(&self) -> &str {
		match self {
			Message => return "message",
			PartialMessage => return "partial",
		}
	}
}
