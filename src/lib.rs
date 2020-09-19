use std::{
    io,
    net::TcpStream,
};
use io::prelude::*;

pub struct Buffer{
    stream: TcpStream,
    cap: usize,
    pos: usize,
    buf: Vec<u8>,
}

const DEFAULT_BUF_SIZE: usize = 2000000;
const DEFAULT_CHUNK_SIZE: usize = 8192;

impl Buffer{
    pub fn new(stream:TcpStream)->Self{
        Self::with_capacity(stream, DEFAULT_BUF_SIZE)
    }

    /// whenever Buffer read completely this function will fill that buffer
    fn fill_buf(&mut self){
        self.buf.resize(self.cap + DEFAULT_CHUNK_SIZE, 0u8);                        // extends the buffer to accept new data
        let read_amount = self.stream.read(&mut self.buf[self.cap..]).unwrap();     // writes to buffer from last point. ie: cap
        self.cap += read_amount;                                                    // extends to new cap size
        self.buf.truncate(self.cap);                                                // fits the buffer to current cap.
    }

    pub fn with_capacity(stream: TcpStream, size: usize)->Self{
        let buf = Vec::with_capacity(size);
        Buffer{
            buf,
            pos:0,
            cap:0,
            stream,
        }
    }

}


impl Read for Buffer{
    fn read(&mut self, buf: &mut[u8])->io::Result<usize>{
        if self.cap <= self.pos{
            self.fill_buf();
        }
        let remain_buf = self.cap - self.pos  ;
        if remain_buf <= buf.len(){
            buf[..remain_buf].clone_from_slice(&self.buf[self.pos .. self.pos + remain_buf]);
            self.pos = self.cap;
            Ok(remain_buf)
        }else{
            buf.clone_from_slice(&self.buf[self.pos ..  self.pos + buf.len()]);
            self.pos = self.pos + buf.len();
            Ok(buf.len())
        }
    }
}

impl Seek for Buffer{
    fn seek(&mut self, from: io::SeekFrom)-> io::Result<u64>{
        let new_pos:usize = match from{
            io::SeekFrom::Start(position) => position as usize,
            io::SeekFrom::Current(position) => position as usize + self.pos,
            io::SeekFrom::End(_) => self.cap
        };
        self.pos = if new_pos < self.cap{ new_pos} else{ self.cap };
        Ok(self.pos as u64)
    }
}
