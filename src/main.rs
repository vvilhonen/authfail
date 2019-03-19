use std::net::{TcpStream, Shutdown};
use std::time::Duration;
use net2::TcpStreamExt;
use std::io::Write;
use std::{env, process};

fn main() {
    let args = env::args().collect::<Vec<_>>();
    if args.len() != 2 {
        eprintln!("Usage: {} <ip:port>", args[0]);
        process::exit(1);
    }
    println!("Connecting to {}", args[1]);
    let mut s = TcpStream::connect(&args[1]).expect("connect");
    s.set_linger(Some(Duration::from_secs(100000))).expect("setsockopt");
    s.write_all(include_bytes!("../req.txt")).unwrap();
    s.shutdown(Shutdown::Both).unwrap();
}