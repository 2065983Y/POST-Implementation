extern crate num;
extern crate serde_json;
extern crate num_traits;

#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate num_derive;

mod iCarrier;
mod message;
mod iSendable;
mod iReceivable;
mod message_handler;

mod dns_client;

use dns_client::DnsClient;

use message_handler::MessageHandler;
use message::Message;

//use dns_client::DnsClient;

impl MessageHandler for DnsClient {
	type Item = String;

	fn on_msg_recv(message: &Message<Self::Item>)
	{
		println!("On msg recv function called");
		println!("Different impl");
		println!("Received msg: {:?}", message);
	}

}

fn main()
{
	let query_addr = String::from("www.google.co.uk");

	let dns_client = DnsClient::new();
	dns_client.query_addr(query_addr);


	println!("it's working jeff");
}


