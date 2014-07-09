#![crate_name = "connect_s3"]

extern crate debug;
extern crate http;
use http::client::RequestWriter;
use http::method::Get;
use http::headers::HeaderEnum;
use std::os;
use std::str;
use std::io::println;

fn main() {
    format!("{}", Get);
    let s3_endpoint = "http://s3.amazonaws.com/";
    let req = connect_s3(s3_endpoint);
    let mut response = match req.read_response() {
        Ok(response) => response,
        Err(_request) => fail!("This example can progress no further with no response :-("),
    };
    println!("{}", response.status);
    println!("Headers:");
    for header in response.headers.iter() {
        println!(" - {}: {}", header.header_name(), header.header_value());
    }
    println!("Body:");
    let body = match response.read_to_end() {
        Ok(body) => body,
        Err(err) => fail!("Reading response failed: {}", err),
    };
    println(str::from_utf8(body.as_slice()).expect("Uh oh, response wasn't UTF-8"));
}

// Uh, I guess get a connection, and once that's in hand, pass it on for e.g. auth?
fn connect_s3(s3_endpoint: &str) -> RequestWriter {
    let request: RequestWriter =
        RequestWriter::new(Get, from_str(s3_endpoint).expect("Invalid URL :-(")).unwrap();
    return request;
}
