// Stores far-end information for a given assosiation

#[derive(Debug)]
pub struct Remote {
	pub hostname: String,
	pub port: i32
}

impl Remote {

	pub fn get_query_addr(&self) -> String {
		format!("{}:{}", self.hostname, self.port)
	}
}
