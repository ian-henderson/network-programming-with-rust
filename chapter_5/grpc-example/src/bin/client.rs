extern crate grpc;
extern crate grpc_example;

use grpc_example::foobar::*;
use grpc_example::foobar_grpc::*;

fn main() {
    // Create a client to talk to a given server
    let client = FooBarServiceClient::new_plain("127.0.0.1", 9001, Default::default()).unwrap();

    let mut req = CabLocationRequest::new();
    req.set_name("foo".to_string());

    let mut location = Location::new();
    location.latitude = 40.730610;
    location.longitude = -73.935242;
    req.set_location(location);

    // First RPC call
    let resp = client.record_cab_location(grpc::RequestOptions::new(), req);
    match resp.wait() {
        Ok((_, r, _)) => println!("{:?}", r),
        Err(error) => eprintln!(":?", error),
    }
    let mut nearby_req = GetCabRequest::new();
    let mut location = Location::new();
    location.latitude = 40.730610;
    location.longitude = -73.935242;
    nearby_req.set_location(location);

    // Another RPC call
    let nearby_resp = client.get_cabs(grpc::RequestOptions::new(), nearby_req);
    match nearby_resp.wait() {
        Ok((_, r, _)) => println!("{:?}", r),
        Err(error) => eprintln!(":?", error),
    }
}
