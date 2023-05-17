use std::fmt::format;
use std::net::TcpStream;

pub struct HttpResponse {
  status_code: u16,
  headers: Vec<(String, String)>,
  body: Vec<u8>,
}

impl HttpResponse {
  pub fn new(status_code: u16) -> Self {
    HttpResponse {
      status_code,
      headers: Vec::new(),
      body: Vec::new(),
    }
  }

  pub fn header (mut self, name: &str, value: &str) -> Self {
    self.headers.push((name.to_string(), value.to_string()));
    self
  }

  pub fn body(mut self, body: &[u8]) -> Self {
    self.body.extend_from_slice(body);
    self
  }

  /*pub fn send (mut self, stream: &mut TcpStream) {

  }*/

  pub fn build (self) -> Vec<u8> {
    let mut response = format!("HTTP/1.1 {} OK\r\n", self.status_code);

    for (name, value) in &self.headers {
      response.push_str(&format!("{}: {}\r\n", name, value));
    }

    response.push_str("\r\n");
    let mut response = response.into_bytes();
    response.extend(self.body);
    response
  }
}