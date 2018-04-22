#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate hyper;
extern crate iron;
extern crate curl;

mod remote;
mod message;
mod iSendable;
mod iReceivable;
mod iCarrier;
mod message_handler;
mod transient;

#[macro_use]
mod type_info;

use hyper::*;
use hyper::client::IntoUrl;
use std::io::Read;
use curl::easy::Easy;

use iCarrier::ICarrier;
use remote::Remote;
use message::Message;
use message::Point;
use iReceivable::IReceivable;
use message_handler::MessageHandler;
use transient::Transient;
use transient::Transport;


use type_info::TypeInfo;
impl_type_info!(i32, i64, f32, f64, str, String, Vec<T>, Message<T>, Point<T>);


use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};
use std::net::TcpStream;
use std::result::Result::Ok;
use std::thread;
use std::time::Duration;
use std::sync::{Arc, Mutex};
use std::sync::mpsc::{ channel, Sender };
use std::str::FromStr;

//temp
use std::io::{stdout, Write};


struct HttpClient<'a> {

	r: &'a Remote,
	//query_addr: String,
	query_addrs: Vec<(String, String)>,
	preferred_addr: Option<IpAddr>,
	client: Client,
	transient: Option<Transient>

}

impl<'a> HttpClient<'a> {

	fn new(r: &'a Remote) -> Self {

		Self {
			r: r,
			query_addrs: r.get_query_addrs(),
		//	query_addrs: addrs,
			preferred_addr: None,
			client: Client::new(),
			transient: None
		}
	}

	fn candidate_send(sender: Sender<RaceResult>, addr_port: (String, String), post_data: Vec<u8>) {

		let mut handle = Easy::new();

		println!("Racing {}", addr_port.0);

		//TODO: remove hardcode
		let mut q_addr = "";		
		if addr_port.0 == "127.0.0.1" {
			//thread::sleep_ms(2000);
			q_addr = "http://127.0.0.1:3005";
		} else {
			//thread::sleep_ms(2000);
			q_addr = "http://[::1]:3000";
		}

//		println!{"http://{}:{}", addr_port.0, addr_port.1}
//		println!("{}", q_addr);

		let res = Self::send_post(post_data);
//		handle.url(q_addr).unwrap();
//		handle.write_function(|data| {
//			Ok(stdout().write(data).unwrap())
//		}).unwrap();
//		handle.perform().unwrap();
		Self::data_recv(res);

		sender.send(RaceResult{ addr: addr_port.0, port: addr_port.1 });
	}

	fn candidate_connect(sender: Sender<TcpStream>, addr_port: (String, String))
    {
        println!("Racing {:?}", addr_port);
 
        let mut to_race = format!("{}:{}", addr_port.0, addr_port.1);
//        if addr_port.0 == "127.0.0.1" {
//            to_race = "127.0.0.1:3005";
//        } else {
//            to_race = "[::1]:3006";
//        }
 
        let mut stream = TcpStream::connect(to_race);
 
        match stream {
            Ok(s) => { sender.send( s ); }
            Err(_) => { println!("Fail...")}
        }
 
 
       
    }


	fn race(alts: Vec<(String, String)>) -> Option<(Transient, TcpStream)> {

		println!("addrs to race: {:?}", alts);	
		let (tx, rx) = channel();


		for addr in alts {
            let tx_clone = tx.clone();
            thread::spawn(move || Self::candidate_connect(tx_clone, addr));
        }
 
        match rx.recv() {
            Ok(stream) =>
            {
				let af = stream.peer_addr().unwrap().ip();
				let transport = Transport::stream;//String::from("Stream");
				let transient = Transient::new(af, transport);
                println!("Winner addr: {:?}", stream.peer_addr().unwrap().ip());
                return Some((transient, stream))
            },
            Err(c) =>
            {
                //TODO: Handle error
                println!("Error at conn racing :( {:?}", c);
                return None;
            },
        }

	}


	fn send_post(bytes: Vec<u8>) -> Vec<u8>
	{
		let (tx, rx): (Sender<Vec<u8>>, _) = channel();
		//let mut res :&[u8] = &[];

//		println!("{:?}", bytes);
		let mut data = bytes.as_slice();	

		let mut easy = Easy::new();


		easy.url("http://127.0.0.1:3005/message").unwrap();
		easy.post(true).unwrap();
		easy.post_field_size(data.len() as u64).unwrap();

		let mut transfer = easy.transfer();

		transfer.read_function(|buf| {
		    Ok(data.read(buf).unwrap_or(0))
		}).unwrap();


		transfer.write_function(|data| {
//			println!("{:?}", data);
			tx.send(data.to_vec());
			//res = data;
//			println!("string result:\\/ \n{}", String::from_utf8(data.to_vec()).unwrap());
			Ok(data.len())
		}).unwrap();

		transfer.perform().unwrap();
		rx.recv().unwrap()

	}

}

impl iSendable::ISendable<Vec<u8>> for Message<Point<i32>> {

	fn encode(&self) -> Vec<u8>
	{
//		println!("Msg in mehtod: {:?}", self);
		let pl = serde_json::to_string(&self).unwrap();
		let b = "{\"data\": {\"x\": 5, \"y\": 42}}";
		//let _: () = b;
//		println!("{}", pl);
//		println!("{}", b);
		pl.as_bytes().to_vec()		
	}
}

impl<'a> ICarrier for HttpClient<'a> {
	type Item=Point<f64>;
	type Transmitter=Vec<u8>;

	fn init(mut self) -> Self {
		let addrs = self.r.get_query_addrs();
		let mut buff = addrs.clone();

		//TODO: temp add addr to carrier
		//TODO: push remotes
		buff.push((String::from("127.0.0.1"), String::from("3005")));

		let (transient, stream) = Self::race(buff).unwrap();
        let pref_addr = Some(stream.peer_addr().unwrap().ip());
		println!("Setting preferred addrs to: {:?}", pref_addr);

		self.preferred_addr = pref_addr;
		self.transient = Some(transient);

		self
	}

	fn send_msg<T>(&mut self, msg: T) where T: iSendable::ISendable<Vec<u8>> {
		let body_str = msg.encode();
//		println!("Decoded msg bytes: {:?}", body_str);

		let s = String::from_utf8(body_str.clone()).unwrap();
//		println!("Decoded msg: {}", s);

		//TODO: check if preffered address is set and use it, o/w race connections

		if self.preferred_addr == None {
			println!("no preferred addr, racing connections...");
			//TODO: remove clones, fix signatures


			let mut b = self.query_addrs.clone();
//			println!("Setting preffered addrs to {:?}", b);

			//TODO: temp add addr to carrier
			//TODO: push remotes
			b.push((String::from("127.0.0.1"), String::from("3005")));
//			b.push(String::from("::1"));

			let (transient, stream) = Self::race(b).unwrap();
			self.transient = Some(transient);
            self.preferred_addr = Some(stream.peer_addr().unwrap().ip());
			println!("Setting preferred addrs to: {:?}", self.preferred_addr);
		}
		else {
			println!("preferred addr is {:?}", self.preferred_addr);
		}

		let post_res = Self::send_post(body_str);
		let mut buff = Vec::new();
		buff.extend(post_res[..10].to_vec());
		Self::data_recv(post_res[..10].to_vec());
		buff.extend(post_res[10..].to_vec());
		//println!("{}, {}", post_res.len(), buff.len());
		Self::data_recv(buff);

	}

	fn data_recv<T>(mut received: T) -> Option<Message<Self::Item>>
	where T: IReceivable<Message<Self::Item>>
	{
//		println!("Called data_recv");

		let res = received.decode();
		match res {
			Some(msg) =>
			{
				//let msg = res.unwrap();
				Self::msg_recv(&msg);
				Some(msg)
			}
			_ => { println!("Could not reassemble message"); None }
		}

	}

	fn msg_recv(message: &Message<Self::Item>) 
	{
//		println!("Message recv called");
		Self::on_msg_recv(message);
	}

}


impl IReceivable<Message<Point<f64>>> for Vec<u8>
{

	fn decode(&mut self) -> Option<Message<Point<f64>>> {
//		println!("Decoding...");

//		println!("{:?}", self);
//		let msg_str = String::from_utf8(self.to_vec()).unwrap();
//		println!("String data {}", msg_str);
//		let data: Message<Point<f64>> = serde_json::from_str(msg_str.as_str()).unwrap();
//		println!("{}", data.data.x);
//		println!("data from string {:?}", data);

		let msg = serde_json::from_slice(self);
		match (msg) {
			Ok(item) => { return Some(item); }
			Err(_) => { return None;}
		}


//		let int_msg = Message {data: Point{ x: msg.data.x as i32, y: msg.data.y as i32}};
//		int_msg
	}
}

impl<'a> MessageHandler for HttpClient<'a> {
	type Item = Point<f64>;


	fn on_msg_recv(message: &Message<Self::Item>)
	{
		println!("on msg recv called");
		println!("{:?}", message);
		println!("Type of message: {}", message.type_of());
	}
}

#[derive(Debug)]
struct RaceResult {
	addr: String,
	port: String
	//TODO: add result

}

impl RaceResult {

	fn get_ip_addr(&self) -> IpAddr {
		let clone = self.addr.clone();
		let mut res = clone.trim_matches('[');
		res = res.trim_matches(']');
//		println!("res {}", res);
		IpAddr::from_str(res).expect(format!("Cannot decode {} as ip address", res).as_str())
	}
}


fn race_thr(alts: Vec<String>) -> Option<IpAddr> {

	println!("addrs to race thr: {:?}", alts);

	let fastest: Option<String> = None;
	let fastest_mutex = Arc::new(Mutex::new(fastest));

	let mut children = vec![];

	for addr in alts {
		let fastest_clone = fastest_mutex.clone();

	    let handle = thread::spawn(move || {
			if *fastest_clone.lock().unwrap() == None {
				//access server		
				//*fastest_clone.lock().unwrap() = //Some(4);
			} else {
				println!("Value is taken");
			}
	    });
		children.push(handle);

	}

	match "hi".parse() {
		Ok(x) => return Some(x),
		Err(_) => return None
	}
}


fn main() {

	//let remote = Remote {preferred: Some(String::from("127.0.0.1")), alternatives: Vec::new(), port: 3005};

	let alternatives = vec![(String::from("[::1]"), String::from("3006"))];

	let remote = Remote::new(Some(String::from("127.0.0.1")), alternatives, 3005);
	let mut http_client = HttpClient::new(&remote);

	let msg = Message::new(Point {x: 5, y: 42});
	http_client.send_msg(msg);

}
