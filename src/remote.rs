// Stores far-end information for a given assosiation
use std::net::ToSocketAddrs;

#[derive(Debug)]
pub struct Remote {
	pub preferred: Option<String>,
	alternatives: Vec<(String, String)>,
	pub port: i32
}

impl Remote {

	pub fn new(preferred: Option<String>, alternatives: Vec<(String, String)>, port: i32) -> Self {
		Self {
			preferred: preferred,
			alternatives: alternatives,
			port: port
		}
	}

	pub fn get_query_addr(&self) -> Option<String> {
		if self.preferred != None {
			let pref_clone = self.preferred.clone();
			return Some(format!("{}:{}", pref_clone.unwrap(), self.port));
		} 
		None
	}

	pub fn get_query_addrs(&self) -> Vec<(String, String)> {
		self.alternatives.clone()
	}

	pub fn new_from_name(domain_name: String) -> Self
	{
		// Ideally, we would want to use std::net::lookup_host to resolve the host ips,
		// but at time of writing this, it is still unstable. Fortunately, we can
		// still resolve using the ToSocketAddrs trait, but we need to add a port,
		// so we use the dummy port 0.
		let host_port = (domain_name.as_str(), 0);
    	let mut ip_iter = host_port.to_socket_addrs().unwrap();
 
		let preferred = Some(format!("{}", ip_iter.next().unwrap().ip()));
		let port = 80;
		let mut alternatives = Vec::new();

		for ip_port in ip_iter {
		    let ip = format!("{}", ip_port.ip());
			alternatives.push( (ip, String::from("80")) );
		}

		Self {
			preferred: preferred,
			alternatives: alternatives,
			port: port
		}
	}
}
