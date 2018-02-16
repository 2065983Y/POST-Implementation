// Trait for carriers to implement
// TODO: needs refactoring
//	-- DONE 1. Add Message events in carrier

extern crate iron;

//use remote::Remote;

use message::Message;
//use message::Point;
use self::iron::prelude::*;
use ISendable::ISendable;

pub trait ICarrier {
	type Item; // Type of messages the carrier will work with
	type Transmitter;
	//fn init(&self, remote: Remote) -> Box<ICarrier>;

	fn data_rcv(request: &mut Request) -> IronResult<Response>;

	fn msg_rcv(message: Message<Self::Item>, f: fn(Message<Self::Item>) );

	fn on_msg_rcv(message: Message<Self::Item>);

	fn send_msg<T>(&self, message: T) where T: ISendable<Self::Transmitter>;

}
