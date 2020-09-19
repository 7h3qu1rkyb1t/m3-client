use m3_client::Buffer;
use std::net::TcpStream;
use rodio::Sink;
use std::io::prelude::*;

fn main() {
    let device = rodio::default_output_device().unwrap();
    let sink = Sink::new(&device);
    let mut stream = TcpStream::connect("127.0.0.1:5533").unwrap();
    stream.write("GET FILE".as_bytes()).unwrap();
    let buffer = Buffer::with_capacity(stream, 8192);
    let source = rodio::Decoder::new(buffer).unwrap();
    sink.append(source);
    sink.sleep_until_end();
}
