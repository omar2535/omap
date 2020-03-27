use std::net::{TcpListener, TcpStream};

fn main() {
    println!("Initializing omap port scanner");
    for port_number in 0..65535 {
        let addr: String = format!("192.168.0.177:{}",port_number);
        if let Ok(_stream) = TcpStream::connect(addr) {
            println!("Port open on: {}", port_number);
        }
    }
    println!("Stopping omap port scanner");
}
