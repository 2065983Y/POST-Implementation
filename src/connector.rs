#[macro_use]
extern crate serde_derive;
extern crate hyper;

use hyper::*;
use std::io::Read;

mod remote;
mod message;
use remote::Remote;

struct HttpClient {

	r: Remote

}


fn main() {
    let client = Client::new();
    let mut res = client.post("http://localhost:3005/message").body("{\"data\": {\"x\": 5, \"y\": 42}}").send().unwrap();
    assert_eq!(res.status, hyper::Ok);
 //   let mut res = client.get("http://localhost:3000/").send().unwrap();
 //  assert_eq!(res.status, hyper::Ok);
    let mut s = String::new();
    res.read_to_string(&mut s).unwrap();
    println!("{}", s);
}
