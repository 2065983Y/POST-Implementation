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
use iCarrier::ICarrier;
use iReceivable;


pub struct HttpListener {

}

impl HttpListener {
	pub fn new() -> Self {
		Self {}
	}

	fn recv(request: &mut Request) -> IronResult<Response>{
		

		//_on_msg_rcv(payload);
//		let mut payload = String::new();
//	    request.body.read_to_string(&mut payload).unwrap();
//		let payload = "fake payload";		

		let data = Self::data_rcv(request);	

		let payload = serde_json::to_string(&data).unwrap();		
		
	    Ok(Response::with((status::Ok, payload)))
	}

}

impl listener::Listener for HttpListener {


	fn listen(&self, local: local::Local) {
		let mut router = Router::new();

		router.post("/message", Self::recv, "message_receipt");


		println!("Listening on port {}", local.port);

		let setup = format!("{}:{}", "localhost", local.port);

		Iron::new(router).http(setup).unwrap();
    	//println!("On 3000");
	}

}

impl<'a, 'b, 'c> iReceivable::IReceivable<Message<Point<i32>>> for &'c mut Request<'a, 'b> {

	fn decode(&mut self) -> Message<Point<i32>> {

		let mut payload = String::new();
	    self.body.read_to_string(&mut payload).unwrap();
		println!("Read: {:?}", payload);
		println!("Received a request from: {:?}", self.remote_addr);		

		serde_json::from_str(&payload).unwrap()
	}
}


impl ICarrier for HttpListener {
	type Item = Point<i32>;
	type Transmitter = Request<'static, 'static>;

	fn data_rcv<T>(mut received: T) -> Message<Self::Item>
where T: iReceivable::IReceivable<Message<Self::Item>> {

		let msg = received.decode();
		Self::msg_rcv(&msg, Self::on_msg_rcv);
		msg
    }

	fn msg_rcv(message: &Message<Self::Item>, f: fn(&Message<Self::Item>)) {
		println!("Received a message");
		f(message);
	}

	fn on_msg_rcv(message: &Message<Self::Item>) {
		println!("{:?}", message);		
	}

	fn send_msg<T>(&self, message: T) {
		

	}


}
