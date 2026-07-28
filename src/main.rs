use std::{
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let mut stream = stream.unwrap();
        stream.write("HTTP/1.1 200 OK\r\n\r\n".as_bytes()).expect("TODO: panic message");
        println!("Connection established");
    }
}
