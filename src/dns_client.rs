use std::net::{Ipv4Addr, UdpSocket};
use std::fs::File;
use std::process::exit;
use std::os::unix::io::AsRawFd;
use std::io::Read;
use std::str::FromStr;
use num::FromPrimitive;

use iCarrier::ICarrier;
use iSendable::ISendable;
use message::Message;
use iReceivable::IReceivable;
use message_handler::MessageHandler;

#[derive(Debug)]
pub struct DnsClient {
	dns_addr: Ipv4Addr,
	local_socket: UdpSocket,
	dns_port: u16
}


impl DnsClient {

	//
	//	PRIVATE Methods
	//

	fn bind_client_socket() -> UdpSocket {
		let client_local_port = "0.0.0.0:65530"; // TODO randomise and retry
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


	fn recv(&self) {
		let mut response_buf = [0; 100];
		//sometimes does not complete....
		let (n, address) = self.local_socket.recv_from(&mut response_buf).unwrap();

		println!("Got {} bytes from {} ", n, address);
		println!("Processing...");
		let mut response_vec: Vec<u8> = Vec::new();
		for &x in response_buf.iter() 
		{
			response_vec.push(x);
		}
		let msg = Message { data: response_vec };
		Self::data_recv(msg);
		println!("Done reconsctructing message. Parsing messsage...");
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
		println!("Client: {:?}", self);
		let query = Message {data: (addr, (7,9)) };
		self.send_msg(query);
	}

}


impl IReceivable<Message<String>> for Message<Vec<u8>> 
{
	fn decode(&mut self) -> Message<String> {
		println!("Decoding...");
		//check messsage id
		let mut iter = self.data.iter();

		//TODO: Hardcoded, replace...
		assert_byte(iter.next(), &(7 as u8), "First byte of response id needs to be the same.");
		assert_byte(iter.next(), &(9 as u8), "Second Byte of response id needs to be the same.");	

		let mut decodedStr = String::new();

		process_byte(iter.next(), |b| {
				decodedStr = format!("{}\n\tIs response: {}", decodedStr, check_single_bit(b, 7));
				decodedStr = format!("{}\n\tStandart Query: {}", decodedStr,
					!(check_single_bit(b, 6) && check_single_bit(b, 5) && check_single_bit(b, 4) && check_single_bit(b, 3)) //TODO: verify
									);
				decodedStr = format!("{}\n\tAuthorative Answer: {}", decodedStr, check_single_bit(b, 2));
				decodedStr = format!("{}\n\tMsg Truncated: {}", decodedStr, check_single_bit(b, 1));
				decodedStr = format!("{}\n\tRecursion Desired: {}", decodedStr, check_single_bit(b, 0));
			}
		);

		process_byte(iter.next(), |b| {
				decodedStr = format!("{}\n\tRecursion available: {}", decodedStr, check_single_bit(b, 7));
				decodedStr = format!("{}\n\tReserved bits preserved: {}", decodedStr, !(check_single_bit(b, 6) && check_single_bit(b, 5) && check_single_bit(b, 4)));
				let rcode = b & 15;
				let rcode_opt :Option<Rcode> = FromPrimitive::from_u8(rcode);
				println!("{:?}", rcode_opt);
				let rcode_obj = rcode_opt.expect(format!("Cannot decode {} as rcode", rcode).as_str());
				if rcode_obj != Rcode::ROk
				{
					panic!();
				}
				decodedStr = format!("{}\n\tRcode: {:?}", decodedStr, rcode_obj);
			}
		);

		decodedStr = format!("{}\n\tQDCOUNT: {}", decodedStr, get_two_byte_value(iter.next(), iter.next()));
 		decodedStr = format!("{}\n\tANCOUNT: {}", decodedStr, get_two_byte_value(iter.next(), iter.next()));
  		decodedStr = format!("{}\n\tNSCOUNT: {}", decodedStr, get_two_byte_value(iter.next(), iter.next()));
  		decodedStr = format!("{}\n\tARCOUNT: {}", decodedStr, get_two_byte_value(iter.next(), iter.next()));

		let mut name_part_byte: u8;
		let mut name = String::new();
  		while {
    		name_part_byte = *iter.next().unwrap();
    		name_part_byte != 0
  		} {
    		let &part_length = &name_part_byte;
    		for _ in 0..part_length {
      			name.push((*iter.next().unwrap()) as char);
    		}
    		name.push('.');
		}
		name.pop();

		decodedStr = format!("{}\n\tQNAME: {}", decodedStr, name);		

		decodedStr = format!("{}\n\tQTYPE: {}", decodedStr, print_type(get_two_byte_value(iter.next(), iter.next()) as u8));
		decodedStr = format!("{}\n\tQCLASS: {}", decodedStr, get_two_byte_value(iter.next(), iter.next()));

  		let first_name_byte = *iter.next().unwrap();
  		let first_name_bit = check_single_bit(&first_name_byte, 7);
  		let second_name_bit = check_single_bit(&first_name_byte, 6);
  		let response_name_is_pointer = first_name_bit && second_name_bit;
  		decodedStr = format!("{}\n\tIs pointer? {}", decodedStr, response_name_is_pointer);
  		
		if response_name_is_pointer {
    		iter.next();
		}
	
		decodedStr = format!("{}\n\tTYPE: {}", decodedStr, print_type(get_two_byte_value(iter.next(), iter.next()) as u8));
  		decodedStr = format!("{}\n\tCLASS: {}", decodedStr, get_two_byte_value(iter.next(), iter.next()));

  		let ttl_byte_1 = *iter.next().unwrap() as u32;
  		let ttl_byte_2 = *iter.next().unwrap() as u32;
  		let ttl_byte_3 = *iter.next().unwrap() as u32;
  		let ttl_byte_4 = *iter.next().unwrap() as u32;
  		let ttl = (ttl_byte_1 << 24) + (ttl_byte_2 << 16) + (ttl_byte_3 << 8) + (ttl_byte_4 << 0);
  		decodedStr = format!("{}\n\tttl: {} {} {} {} {}", decodedStr, ttl_byte_1, ttl_byte_2, ttl_byte_3, ttl_byte_4, ttl);

  		let rdlength = get_two_byte_value(iter.next(), iter.next());
  		decodedStr = format!("{}\n\trdlength: {}", decodedStr, rdlength);

  		decodedStr = format!("{}\n\tRESPONSE: ", decodedStr);
		let mut ip = String::new();
  		for _ in 0..rdlength {
    		ip = format!("{}{}.", ip, *iter.next().unwrap());
		}
		ip.pop();
		decodedStr = format!("{}\n\t{}", decodedStr, ip);

		println!("Raw data: {:?}", self);
		println!("Devoded: {}", decodedStr);

		Message{ data: format!("{}", decodedStr) }
	}

}

#[derive(Debug, PartialEq, FromPrimitive)]
enum Rcode {
  ROk = 0,
  FormatError = 1,
  ServerFailure = 2,
  NameError = 3,
  NotImplemented = 4,
  Refused = 5
}

#[derive(Debug, PartialEq, FromPrimitive)]
enum RecordType {
  A = 1,
  NS = 2,
  CNAME = 5,
  SOA = 6,
  WKS = 11,
  PTR = 12,
  MX = 15,
  SRV = 33,
  A6 = 38
}

fn print_type(type_code: u8) -> String 
{
	let t: RecordType = FromPrimitive::from_u8(type_code).expect(format!("{} is an unknown type", type_code).as_str());
	format!("{:?}", t)
}

fn process_byte<F>(byte_opt: Option<&u8>, mut processor: F)
	where F: FnMut(&u8)
{
	let b = byte_opt.expect("Option is empty");
	processor(b)
}


fn check_single_bit(b: &u8, position: u32) -> bool
{
	b & (1 << position) != 0
}


fn assert_byte(actual: Option<&u8>, expected: &u8, msg: &str) 
{
	process_byte(actual, |b| {
			if expected != b
			{
				println!("{} Expected {} but was {}", msg, expected, b);
				panic!();
			}
		}
	);		
}

fn get_two_byte_value(msb: Option<&u8>, lsb: Option<&u8>) -> u32 {
	(*msb.unwrap() as u32) << 8 | (*lsb.unwrap() as u32)	
}




impl ISendable<Vec<u8>> for Message<(String, (u8, u8))> 
{
	type Item=(String, (u8,u8));

	fn encode(&self) -> Vec<u8>
	{
		let mut result: Vec<u8> = Vec::new();
		result.push((&self.data.1).0); // message id 1
		result.push((&self.data.1).1); // message id 2
		result.push(0x01); // qr, opcode, aa, tc, rd
		result.push(0x00); // ra, res1, res2, res3, rcode
		result.push(0x00); // qdcount 1
		result.push(0x01); // qdcount 2
		result.push(0x00); // ancount 1
		result.push(0x00); // ancount 2
		result.push(0x00); // nscount 1
		result.push(0x00); // nscount 2
		result.push(0x00); // arcount 1
		result.push(0x00); // arcount 2

		for p in (&self.data.0).split(".") {
		  result.push(p.as_bytes().len() as u8); // length
		  for &c in p.as_bytes() {
		    result.push(c as u8); // query
		  }
		}
		result.push(0x00); // end name

		result.push(0x00); // qtype 1
		result.push(0x01); // qtype 2
		result.push(0x00); // qclass 1
		result.push(0x01); // qclass 2
		result	
	}
}





impl ICarrier for DnsClient {
	type Item = String;
	type Transmitter = Vec<u8>;

	// TODO: can we get around passing mut arg?
	fn data_recv<T>(mut received: T) -> Message<Self::Item>
		where T: IReceivable<Message<Self::Item>>
	{
		println!("Called recv");
		let res = received.decode();
		Self::msg_recv(&res);
		res
	}

	fn msg_recv(message: &Message<Self::Item>)
	{
		println!("Received a msg, calling on_msg_recv");
		Self::on_msg_recv(message);
	}

	fn send_msg<T>(&self, message: T) where T: ISendable<Self::Transmitter>
	{
		let encoded = message.encode();
		println!("{:?}", encoded);

		let bytes_written = &self.local_socket.send_to(&encoded, (self.dns_addr, self.dns_port)).expect("Failed to send DNS request");

		println!("Wrote {} bytes", bytes_written);

		//TODO: is call to recv sensible here?
		self.recv();
	}


}



//fn main()
//{
//	let query_addr = String::from("www.google.co.uk");

//	let dns_client = DnsClient::new();
//	dns_client.query_addr(query_addr);


//	println!("it's working jeff");
//}
