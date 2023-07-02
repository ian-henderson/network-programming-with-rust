extern crate trust_dns_proto;
extern crate trust_dns_resolver;
use std::{env, process};
use trust_dns_proto::rr::record_type::RecordType;
use trust_dns_resolver::{config::*, Resolver};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Please provide a name to query");
        process::exit(1);
    }

    let resolver = Resolver::new(ResolverConfig::default(), ResolverOpts::default()).unwrap();

    // add a dot to the given name
    let query = format!("{}.", args[1]);

    // run the dns query
    let response = resolver.lookup_ip(query.as_str());
    println!("Using the synchronous resolver");
    for answer in response.iter() {
        println!("{:?}", answer);
    }

    println!("Using the system resolver");
    let system_resolver = Resolver::from_system_conf().unwrap();
    let system_response = system_resolver.lookup_ip(query.as_str());
    for answer in system_response.iter() {
        println!("{:?}", answer);
    }

    let ns = resolver.lookup(query.as_str(), RecordType::NS);
    println!("NS records using the synchronous resolver");
    for answer in ns.iter() {
        println!("{:?}", answer);
    }
}
