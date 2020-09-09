use std::io::prelude::*;
use std::io;
use std::net::TcpStream;
use std::ops::Deref;
use rodio::Sink;

/// rodio source needs seek functionality but to implement it to TCP stream
struct TcpStreamWrapper(TcpStream);
impl Seek for TcpStreamWrapper{
    fn seek(&mut self, _: io::SeekFrom)->io::Result<u64>{
        Ok(0)
    }
}

impl Read for TcpStreamWrapper{
    fn read(&mut self, buf:&mut[u8])-> std::io::Result<usize>{
        self.0.read(buf)
    }
}

/// so that all the methods of TCPStream are available to wrapper
impl Deref for TcpStreamWrapper{
    type Target = TcpStream;
    fn deref(&self)->&TcpStream{
        &self.0
    }
}

fn main() {
    let device = rodio::default_output_device().unwrap();
    let sink = Sink::new(&device);
    let stream = TcpStream::connect("127.0.0.1:5533").unwrap();
    let buf_reader = TcpStreamWrapper(stream);
    let source = rodio::Decoder::new(buf_reader).unwrap();
    // while let Ok(size) = stream.read(&mut buffer){
    //     source = rodio::Decoder::new(buffer).unwrap();
    // }
    sink.append(source);
    sink.sleep_until_end();
}
