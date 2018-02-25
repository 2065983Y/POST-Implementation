use message::Message;

pub trait MessageHandler {
	type Item;

	fn on_msg_recv(message: &Message<Self::Item>);
}
