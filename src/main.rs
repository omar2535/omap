extern crate clap;

use clap::{Arg, App};
use std::net::{TcpListener, TcpStream};

fn main() {
    let matches = App::new("Omap")
                          .version("1.0")
                          .author("Omar T. <omar2535@alumni.ubc.ca>")
                          .about("Scans open ports at specified address")
                          .arg(Arg::with_name("address")
                              .short("-a")
                              .long("address")
                              .help("Sets the address to be scanned")
                              .takes_value(true))
                          .get_matches();
    println!("Initializing omap port scanner");
    // unwrap to convert option enum to string
    let address = matches.value_of("address").unwrap();
    println!("Starting scan for address: {}", address);
    scan();
    println!("Stopping omap port scanner");
}

fn scan() {
    for port_number in 0..65535 {
        let addr: String = format!("192.168.0.177:{}",port_number);
        if let Ok(_stream) = TcpStream::connect(addr) {
            println!("Port open on: {}", port_number);
        }
    }
}

