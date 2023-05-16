use std::net::ToSocketAddrs;

pub trait Connection {
  fn open<A: ToSocketAddrs>(addr: A) -> Result<Self, std::io::Error> where Self: Sized;
  fn close (&mut self) -> Result<(), std::io::Error>;
  fn send_packet (&mut self, packet: &[u8]) -> Result<(), std::io::Error>;
  fn recv_packet (&mut self) -> Result<Vec<u8>, std::io::Error>;
}