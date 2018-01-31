// TODO:
// Provide support for partial messages

#[derive(Serialize, Deserialize, Debug)]
pub struct Message<T> {
    data: T
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Point<U> {
	x: U,
	y: U
}

