// Driver method, containing examples of specific implementations

#[macro_use]
extern crate serde_derive;


mod remote;
mod carrier;
mod message;

mod http_listener;
mod local;
mod listener;
mod ICarrier;
mod protocol_handler;

use listener::Listener;

fn main() {
//	let remote = remote::Remote { hostname: "localhost".to_string(), port: 3000}; 
	//let http_carier = http_carier::init();
//	println!("{:?}", remote);

	let local = local::Local {port: 3005};
	let listener = http_listener::HttpListener::new();	
	listener.listen(local);
}
