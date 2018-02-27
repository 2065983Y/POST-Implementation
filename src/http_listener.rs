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
use message_handler::MessageHandler;
use iReceivable::IReceivable;

type myType = Point<f32>;

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

		let mut body_data = Vec::new();
		request.body.read_to_end(&mut body_data);

		let data = Self::data_recv(body_data);	

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

impl<'a, 'b, 'c> IReceivable<Message<myType>> for Vec<u8> {

	fn decode(&mut self) -> Message<myType> {

		println!("{:?}", self);

		let v: Message<myType> = serde_json::from_slice(self).unwrap();
		println!("{:?}", v);

		v
	}
}


impl ICarrier for HttpListener {
	type Item = myType;
	type Transmitter = Message<Vec<u8>>;

	fn data_recv<T>(mut received: T) -> Message<Self::Item>
where T: iReceivable::IReceivable<Message<Self::Item>> {

		let msg = received.decode();
		Self::msg_recv(&msg);
		msg
    }

	fn msg_recv(message: &Message<Self::Item>) {
		println!("Received a message");
		Self::on_msg_recv(message);
	}

	//fn on_msg_rcv(message: &Message<Self::Item>) {
	//	println!("{:?}", message);		
	//}

	fn send_msg<T>(&self, message: T) {
		

	}
}

impl MessageHandler for HttpListener {
	type Item = myType;

	fn on_msg_recv(message: &Message<Self::Item>) 
	{
		println!("On Msg Recv fn called with msg: {:?}", message);
	}
}
