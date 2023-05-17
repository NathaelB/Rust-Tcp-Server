use std::io::{Read, Write};
use std::net::{Shutdown, TcpStream, ToSocketAddrs};
use crate::database::result::Error;
use crate::database::connection::Connection;

pub struct RawConnection {
  stream: TcpStream
}

impl Connection for RawConnection {
  fn open<A: ToSocketAddrs>(addr: A) -> Result<Self, std::io::Error> where Self: Sized {
    let stream = TcpStream::connect(addr);

    match stream {
      Ok(stream) => Ok(Self { stream }),
      Err(e) => Err(e)
    }
  }

  fn close (&mut self) -> Result<(), std::io::Error> {
    match self.stream.shutdown(Shutdown::Both) {
      Ok(_) => Ok(()),
      Err(e) => Err(e)
    }
  }

  fn send_packet (&mut self, packet: &[u8]) -> Result<(), std::io::Error> {
    let packet_len = (packet.len() + 4) as i32;
    let mut buf = Vec::new();

    buf.extend(&(packet_len as i32).to_be_bytes());
    buf.extend(packet);

    self.stream.write_all(&buf)?;
    Ok(())
  }

  fn recv_packet (&mut self) -> Result<Vec<u8>, std::io::Error> {
    let mut len_buf = [0u8; 4];
    self.stream.read_exact(&mut len_buf)?;
    let packet_len = i32::from_be_bytes(len_buf) as usize - 4;

    let mut buf = vec![0u8; packet_len];
    self.stream.read_exact(&mut buf)?;

    Ok(buf)
  }
}