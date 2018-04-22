
use std::net::{IpAddr, TcpStream, UdpSocket};

pub struct Transient {

	address_family: IpAddr,
	transport: Transport
}

//pub struct Stream {}
//pub struct Datagram {}

pub enum Transport{
	stream,
	datagram
}

impl Transient {

	pub fn new(address_family: IpAddr, transport: Transport) -> Transient
	{
		Self {
			address_family: address_family,
			transport: transport
		}
	}
}
