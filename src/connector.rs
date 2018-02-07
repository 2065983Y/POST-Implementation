#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate hyper;

use hyper::*;
use std::io::Read;

mod remote;
mod message;
use remote::Remote;


use message::Message;
use message::Point;

struct HttpClient {

	r: Remote

}


fn main() {
	let msg = Message { data: Point {x: 5, y: 42} };
	let pl = serde_json::to_string(&msg).unwrap();
	let b = "{\"data\": {\"x\": 5, \"y\": 42}}";
	//let _: () = b;
	println!("{}", pl);
	println!("{}", b);

    let client = Client::new();
    let mut res = client.post("http://localhost:3005/message").body(pl.as_str()).send().unwrap();
    assert_eq!(res.status, hyper::Ok);
 //   let mut res = client.get("http://localhost:3000/").send().unwrap();
 //  assert_eq!(res.status, hyper::Ok);
    let mut s = String::new();
    res.read_to_string(&mut s).unwrap();
    println!("{}", s);
}
