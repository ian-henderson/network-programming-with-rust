extern crate pom;
use pom::{
    parser::{one_of, seq, sym, Parser},
    DataInput,
};
use std::str;

// Represents one or more occurences of an empty string
fn space() -> Parser<'static, u8, ()> {
    one_of(b" \t\r\n").repeat(0..).discard()
}

// Represents a string in all lower case
fn string() -> Parser<'static, u8, String> {
    one_of(b"abcdefghijklmnopqrstuvwxyz")
        .repeat(0..)
        .convert(String::from_utf8)
}

fn main() {
    let get = b"GET /home/ HTTP/1.1\r\n";

    let mut input = DataInput::new(get);

    let parser = (seq(b"GET") | seq(b"POST"))
        * space()
        * sym(b'/')
        * string()
        * sym(b'/')
        * space()
        * seq(b"HTTP/1.1");

    let output = parser.parse(&mut input);

    println!("{:?}", str::from_utf8(&output.unwrap()));
}
