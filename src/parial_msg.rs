struct PartialMessage {
	uid: i64,
	successors: Vec<i64>,
	chunks: u64,
	data: Vec<u8>
}
