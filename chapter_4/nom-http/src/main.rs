#[macro_use]
extern crate nom;
use nom::{ErrorKind, IResult};
use std::str;

#[derive(Debug)]
enum Method {
    GET,
    POST,
}

#[derive(Debug)]
struct Request {
    method: Method,
    url: String,
    version: String,
}

// A parser that parses method out of a HTTP request
named!(
    parse_method<&[u8], Method>,
    return_error!(
        ErrorKind::Custom(12),
        alt!(
            map!(tag!("GET"), |_| Method::GET) | map!(tag!("POST"), |_| Method::POST)
        )
    )
);

// A parser that parses the request part
named!(
    parse_request<&[u8], Request>,
    ws!(
        do_parse!(
            method: parse_method >>
            url: map_res!(take_until!(" "), str::from_utf8) >>
            tag!("HTTP/") >>
            version: map_res!(take_until!("\r"), str::from_utf8) >>
            (Request { method, url: url.into(), version: version.into() })
        )
    )
);

// Driver function for running the overall parser
fn run_parser(input: &str) {
    match parse_request(input.as_bytes()) {
        IResult::Done(rest, value) => println!("Rest: {:?} Value: {:?}", rest, value),
        IResult::Error(error) => eprintln!("Error: {:?}", error),
        IResult::Incomplete(needed) => println!("{:?}", needed),
    }
}

fn main() {
    let get = "GET /home/ HTTP/1.1\r\n";
    run_parser(get);

    let post = "POST /update/ HTTP/1.1\r\n";
    run_parser(post);

    let wrong = "WRONG /wrong/ HTTP/1.1\r\n";
    run_parser(wrong);
}
