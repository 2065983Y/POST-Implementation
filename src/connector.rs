#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate hyper;
extern crate iron;

mod remote;
mod message;
mod iSendable;
mod iReceivable;
mod iCarrier;

use hyper::*;
use hyper::client::IntoUrl;
use std::io::Read;

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

impl iSendable::ISendable<Vec<u8>> for Message<Point<i32>> {
	type Item=Point<i32>;

	fn encode(&self) -> Vec<u8>
	{
		println!("Msg in mehtod: {:?}", self);
		let pl = serde_json::to_string(&self).unwrap();
		let b = "{\"data\": {\"x\": 5, \"y\": 42}}";
		//let _: () = b;
		println!("{}", pl);
		println!("{}", b);
		pl.as_bytes().to_vec()		
	}
}

impl<'a> ICarrier for HttpClient<'a> {
	type Item=Point<i32>;
	type Transmitter=Vec<u8>;

	fn send_msg<T>(&self, msg: T) where T: iSendable::ISendable<Vec<u8>> {
		let body_str = msg.encode();
		println!("Decoded msg bytes: {:?}", body_str);

		let s = String::from_utf8(body_str).unwrap();
		println!("Decoded msg: {}", s);

		println!("{:?}", self.query_addr);
		let addr = format!("http://{}/message", self.query_addr);
		//let alt = "http://::1:3005/message".into_url();
		//println!("{:?}", alt);
		let mut res = self.client.post(addr.as_str()).body(s.as_str()).send().unwrap();
		//let mut res = self.client.post(alt.unwrap()).body(s.as_str()).send();
		//println!("{:?}", res);
		
		//received msg functionality
		//let mut res1 = res.unwrap();
		assert_eq!(res1.status, hyper::Ok);
		let mut s = String::new();
		res.read_to_string(&mut s).unwrap();
		
		println!("Response after send contained: {}", s);		

	}

	fn data_recv<T>(request: T) -> Message<Self::Item>
	{
		unimplemented!();
	}

	fn msg_recv(message: &Message<Self::Item>) 
	{
		unimplemented!();		
	}


}


fn main() {

	let remote = Remote {hostname: String::from("127.0.0.1"), port: 3005};
	let http_client = HttpClient::new(&remote);

	let msg = Message { data: Point {x: 5, y: 42} };
	http_client.send_msg(msg);

}
