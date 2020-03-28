extern crate clap;

use clap::{Arg, App};
use std::net::TcpStream;

fn main() {

    let matches = generate_cli_input();
    println!("Initializing omap port scanner");
    // unwrap to convert option enum to string
    let address = matches.value_of("address").unwrap();
    println!("Starting scan for address: {}", address);
    scan(&address);
    println!("Stopping omap port scanner");
}

fn generate_cli_input() -> clap::ArgMatches<'static> {
    let matches = App::new("Omap")
                          .version("1.0")
                          .author("Omar T. <omar2535@alumni.ubc.ca>")
                          .about("Scans open ports at specified address")
                          .arg(Arg::with_name("address")
                            .short("-a")
                            .long("address")
                            .help("Sets the address to be scanned")
                            .required(true)
                            .takes_value(true))
                          .arg(Arg::with_name("v")
                            .short("-v")
                            .multiple(true)
                            .help("Sets the level of verbosity"))
                          .get_matches();
    return matches;
}

fn scan(address: &str) {
    for port_number in 0..65535 {
        let addr: String = format!("{}:{}", address, port_number);
        println!("Scanning on: {}", addr);
        if let Ok(_stream) = TcpStream::connect(addr) {
            println!("Port open on: {}", port_number);
        }
    }
}
