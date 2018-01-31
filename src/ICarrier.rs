// Trait for carriers to implement
// TODO: needs refactoring
//	-- DONE 1. Add Message events in carrier

extern crate iron;

//use remote::Remote;

use message::Message;
//use message::Point;
use self::iron::prelude::*;

pub trait ICarrier {
	type Item; // Type of messages the carrier will work with
	//fn init(&self, remote: Remote) -> Box<ICarrier>;

	fn data_rcv(request: &mut Request) -> IronResult<Response>;

	fn msg_rcv(message: Message<Self::Item>, f: fn(Message<Self::Item>) );

	fn on_msg_rcv(message: Message<Self::Item>);

}
