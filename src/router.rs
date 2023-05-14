use std::collections::HashMap;
use std::io::Write;
use std::net::TcpStream;

type Handler = fn(&mut TcpStream);

enum Route {
  Static(String),
  Parameter(String)
}

#[derive(Clone)]
pub(crate) struct Router {
  routes: HashMap<String, Handler>
}

impl Router {
  pub fn new () -> Router {
    Router {
      routes: HashMap::new()
    }
  }

  pub fn add_route (&mut self, path: &str, handler: Handler) {
    self.routes.insert(path.to_string(), handler);
  }

  pub fn handle_request(&self, path: &str, stream: &mut TcpStream) {
    match self.routes.get(path) {
      Some(handler) => {
        handler(stream)
      }
      None => {
        println!("ERROR 404");
        stream.write(b"404 Not Found").unwrap();
        stream.flush().unwrap();
      }
    }
  }
}