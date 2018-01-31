// A concrete implementation for listener, using HTTP connections
// TODO: needs refactoring
//	-- DONE 1. Move Message to another crate 
//	-- DONE 2. Fix logic, listener should not set on_msg_rcv function
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
use message::Message;
use message::Point;
use ICarrier::ICarrier;


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

		Iron::new(router).http(setup).unwrap();
    	//println!("On 3000");
	}

}


impl ICarrier for HttpListener {
	type Item = Point<i32>;

	fn data_rcv(request: &mut Request) -> IronResult<Response> {
		    let mut payload = String::new();
		    request.body.read_to_string(&mut payload).unwrap();
			println!("Read: {:?}", payload);
			println!("Received a request from: {:?}", request.remote_addr);

			let msg: Message<Self::Item> = serde_json::from_str(&payload).unwrap();
			
			Self::msg_rcv(msg, Self::on_msg_rcv);
			//_on_msg_rcv(payload);			
			
		    Ok(Response::with((status::Ok, payload)))
    }

	fn msg_rcv(message: Message<Self::Item>, f: fn(Message<Self::Item>)) {
		println!("Received a message");
		f(message);
	}

	fn on_msg_rcv(message: Message<Self::Item>) {
		println!("{:?}", message);		
	}


}
