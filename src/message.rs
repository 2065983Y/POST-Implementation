// TODO:
// Provide support for partial messages
extern crate serde_json;

use self::serde_json::{Value, Error};


#[derive(Serialize, Deserialize, Debug)]
pub struct Message<T> {
	uid: i64,
	successors: Vec<i64>,
	is_partial: bool,
	chunks: u64,
	is_immediate: bool,
	is_idempotent: bool,
	lifetime: u32,
    data: T,
	extra_fields: Value
}

impl <T> Message<T> {
	pub fn new(data: T) -> Message<T>
	{
		Message {
			uid: 0,
			successors: Vec::new(),
			is_partial: false,
			chunks: 0,
			is_immediate: false,
			is_idempotent: false,
			lifetime: 0,
			data: data,
			extra_fields: serde_json::from_str("{}").expect("Cannot create extra fields json")
		}
	}

	pub fn get_data(&self) -> &T
	{
		&self.data
	}

	pub fn is_partial(&self) -> bool
	{
		*&self.is_partial
	}

	pub fn set_partial(&mut self, val: bool)
	{
		self.is_partial = val;
	}

	pub fn set_fields(&mut self, fields: Value) {
		self.extra_fields = fields;
	}
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Point<U> {
	pub x: U,
	pub y: U
}

