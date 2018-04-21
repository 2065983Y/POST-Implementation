
use std::net::{IpAddr, TcpStream, UdpSocket};

pub struct Transient {

	address_family: String,
	transport: String
}

impl Transient {

	pub fn new(address_family: String, transport: String) -> Transient
	{
		Self {
			address_family: address_family,
			transport: transport
		}
	}

}
