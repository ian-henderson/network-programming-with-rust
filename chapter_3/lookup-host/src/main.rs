use dns_lookup::lookup_host;
use std::{env, process};

fn main() {
    let args: Vec<_> = env::args().collect();

    if args.len() != 2 {
        if args.len() < 2 {
            eprintln!("Please provide a host name, eg: google.com");
        } else {
            eprintln!("Please provide only one host name, eg: google.com");
        }
        process::exit(1);
    }

    let addresses = lookup_host(&args[1]).unwrap();
    for address in addresses {
        println!("{}", address);
    }
}
