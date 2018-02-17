#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate hyper;
extern crate iron;

mod remote;
mod message;
mod ISendable;
mod iReceivable;
mod iCarrier;

use hyper::*;
use std::io::Read;
use iron::prelude::*;

use iCarrier::ICarrier;
use remote::Remote;
use message::Message;
use message::Point;


struct HttpClient<'a> {

	r: &'a Remote,
	query_addr: String,
	client: Client

}

impl<'a> HttpClient<'a> {

	fn new(r: &'a Remote) -> Self {

		Self {
			r: r,
			query_addr: r.get_query_addr(),
			client: Client::new()
		}
	}
}

impl ISendable::ISendable<String> for Message<Point<i32>> {
	type Item=Point<i32>;

	fn encode(&self) -> String
	{
		println!("Msg in mehtod: {:?}", self);
		let pl = serde_json::to_string(&self).unwrap();
		let b = "{\"data\": {\"x\": 5, \"y\": 42}}";
		//let _: () = b;
		println!("{}", pl);
		println!("{}", b);
		pl		
	}
}

impl<'a> ICarrier for HttpClient<'a> {
	type Item=Point<i32>;
	type Transmitter=String;

	fn send_msg<T>(&self, msg: T) where T: ISendable::ISendable<String> {
		let body_str = msg.encode();
		println!("Decoded msg: {}", body_str);

		println!("{:?}", self.query_addr);
		let addr = format!("http://{}/message", self.query_addr);
		let mut res = self.client.post(addr.as_str()).body(body_str.as_str()).send().unwrap();
		//received msg functionality
		assert_eq!(res.status, hyper::Ok);
		let mut s = String::new();
		res.read_to_string(&mut s).unwrap();
		
		println!("Response after send contained: {}", s);		

	}

	fn data_rcv<T>(request: T) -> Message<Self::Item>
	{
		unimplemented!();
	}

	fn msg_rcv(message: &Message<Self::Item>, f: fn(&Message<Self::Item>) ) 
	{
		unimplemented!();		
	}

	fn on_msg_rcv(message: &Message<Self::Item>)
	{
		unimplemented!();		
	}

}


fn main() {

	let remote = Remote {hostname: String::from("localhost"), port: 3005};
	let httpClient = HttpClient::new(&remote);

	let msg = Message { data: Point {x: 5, y: 42} };
	httpClient.send_msg(msg);

}
