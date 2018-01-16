// Trait for listeners to implement
// TODO: refactor
//	1. Abstract message related methods to Carrier

extern crate iron;

use local;
use listener::iron::prelude::*;

use http_listener::Message;
use http_listener::Point;

pub trait Listener {
	fn listen(&self, local: local::Local);

	fn data_rcv(request: &mut Request) -> IronResult<Response>;

	fn msg_rcv(message: Message<Point<i32>>, f: fn(Message<Point<i32>>) );

	fn on_msg_rcv(message: Message<Point<i32>>);
}

