extern crate clap;

use clap::{Arg, App};
use std::net::TcpStream;
use std::thread;

fn main() {
    let matches = generate_cli_input();
    println!("Initializing omap port scanner");
    // unwrap to convert option enum to string
    let address = matches.value_of("address").unwrap();
    println!("Starting scan for address: {}", address);
    scan(&address, matches.is_present("verbose"));
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
                          .arg(Arg::with_name("verbose")
                            .short("-v")
                            .long("verbose")
                            .help("Sets the level of verbosity"))
                          .get_matches();
    return matches;
}

fn scan(address: &str, with_verbosity: bool) {
    for port_number in 0..65535 {
        let addr: String = format!("{}:{}", address, port_number);
        thread::spawn(move || {
            if with_verbosity {println!("Scanning on: {}", addr)};             
            if let Ok(_stream) = TcpStream::connect(addr) {
                println!("Port open on: {}", port_number);
            }
        });
    }
}
