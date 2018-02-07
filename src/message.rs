// TODO:
// Provide support for partial messages

#[derive(Serialize, Deserialize, Debug)]
pub struct Message<T> {
    pub data: T
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Point<U> {
	pub x: U,
	pub y: U
}

