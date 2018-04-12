// Stores far-end information for a given assosiation

#[derive(Debug)]
pub struct Remote {
	pub preferred: Option<String>,
	alternatives: Vec<String>,
	pub port: i32
}

impl Remote {

	pub fn new(preferred: Option<String>, alternatives: Vec<String>, port: i32) -> Self {
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

	pub fn get_query_addrs(&self) -> Vec<String> {
		Vec::new()
	}
}
