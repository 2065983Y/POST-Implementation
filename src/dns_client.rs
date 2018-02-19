#[macro_use]
extern crate serde_derive;
extern crate serde_json;


mod iCarrier;
mod message;
mod iSendable;
mod iReceivable;


use std::net::{Ipv4Addr, UdpSocket};
use std::fs::File;
use std::process::exit;
use std::os::unix::io::AsRawFd;
use std::io::Read;
use std::str::FromStr;

use iCarrier::ICarrier;
use iSendable::ISendable;
use message::Message;
use iReceivable::IReceivable;

struct Dns_client {
	dns_addr: Ipv4Addr,
	local_socket: UdpSocket,
	dns_port: i32
}


impl Dns_client {

	//
	//	PRIVATE Methods
	//

	fn bind_client_socket() -> UdpSocket {
		let client_local_port = "127.0.0.1:65530"; // TODO randomise and retry
		let udp_socket = UdpSocket::bind(client_local_port).ok().
			expect(format!("Could not bind UDP socket to {}", client_local_port).as_str());

		println!("Bound client UDP socket {}", client_local_port);
		Self::set_socket_timeout(&udp_socket);

		udp_socket
	}

	fn set_socket_timeout(socket: &UdpSocket) {
		let _ = socket.set_ttl(1);
		let _ /* raw_fd */  = socket.as_raw_fd();
		//setsockopt(raw_fd.as_sock_t(), SO_RCVTIMEO, 1000, 1000);
	}

	fn read_nameserver() -> String 
	{
		match File::open("/etc/resolv.conf") {
    		Ok(file) => Self::parse_resolv_conf(file),
    		Err(e) => {
      			println!("Could not read /etc/resolv.conf : {}", e);
      			exit(6);
    		}
  		}
	}


	fn parse_resolv_conf(file: File) -> String 
	{
		let mut s = String::new();
		let mut f = file;
		match f.read_to_string(&mut s) {
			Ok(n) => println!("Read {} bytes from file.", n),
			Err(e) => {
				println!("Could not read data from file : {}", e);
				exit(6);
			}
  		}

		let ns_lines = s.split("\n").filter(|&l| l.starts_with("nameserver"));
		let mut ns_addresses = ns_lines.flat_map(|l| l.split_whitespace().skip(1).next());
		return ns_addresses.next().map(|x| x.to_string()).
		    expect((format!("Could find read name server from {}", s).as_str()));
	}


	fn parse_ipv4_addr(src: String) -> Ipv4Addr {
		Ipv4Addr::from_str(src.as_str()).ok()
			.expect(format!("Could not parse ipv4 address from '{}', e", src).as_str())
	}


	//
	//	PUBLIC Methods
	//

	pub fn new() -> Self {
		Self {
			dns_addr: Self::parse_ipv4_addr(Self::read_nameserver()),
			local_socket: Self::bind_client_socket(),
			dns_port: 53
		}
	}

	pub fn query_addr(&self, addr: String) {
		println!("Querying {}", addr);
		
		//TODO:
		// call ICarrier send
	}

}


impl ICarrier for Dns_client {
	type Item = Vec<u8>;
	type Transmitter = Vec<u8>;


	fn data_rcv<T>(received: T) -> Message<Self::Item>
		where T: IReceivable<Message<Self::Item>>
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

	fn send_msg<T>(&self, message: T) where T: ISendable<Self::Transmitter>
	{
		unimplemented!();
	}

}

fn main()
{
	let query_addr = String::from("www.google.co.uk");

	let dns_client = Dns_client::new();
	dns_client.query_addr(query_addr);


	println!("it's working jeff");
}
