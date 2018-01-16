// A concrete implementation for listener, using HTTP connections
// TODO: needs refactoring
//	1. Move Message to another crate
//	2. Fix logic, listener should not set on_msg_rcv function
//	3. Make listener generic, using type params


extern crate iron;
extern crate router;
extern crate serde_json;

use local;
use listener;
use self::router::Router;
use http_listener::iron::prelude::*;
use http_listener::iron::status;
use std::io::Read;

#[derive(Serialize, Deserialize, Debug)]
pub struct Message<T> {
    data: T
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Point<U> {
	x: U,
	y: U
}


pub struct HttpListener {

}

impl HttpListener {
	pub fn new() -> Self {
		Self {}
	}

}

impl listener::Listener for HttpListener {
	fn listen(&self, local: local::Local) {
		let mut router = Router::new();

		router.post("/message", Self::data_rcv, "message_receipt");


		println!("Listening on port {}", local.port);

		let setup = format!("{}:{}", "localhost", local.port);
		println!("{}", setup);

		Iron::new(router).http(setup).unwrap();
    	println!("On 3000");
	}

	fn data_rcv(request: &mut Request) -> IronResult<Response> {
		    let mut payload = String::new();
		    request.body.read_to_string(&mut payload).unwrap();
			println!("Read: {:?}", payload);

			let msg: Message<Point<i32>> = serde_json::from_str(&payload).unwrap();
			
			Self::msg_rcv(msg, Self::on_msg_rcv);
			//_on_msg_rcv(payload);			
			
		    Ok(Response::with((status::Ok, payload)))
    }

	fn msg_rcv(message: Message<Point<i32>>, f: fn(Message<Point<i32>>)) {
		println!("Received a message");
		f(message);
	}

	fn on_msg_rcv(message: Message<Point<i32>>) {
		println!("{:?}", message);		
	}

}
