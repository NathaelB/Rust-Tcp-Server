use std::collections::HashMap;
use std::net::TcpStream;

type Handler = fn(&mut TcpStream);

pub(crate) struct Router {
  routes: HashMap<String, Handler>
}

impl Router {
  pub fn new () -> Router {
    Router {
      routes: HashMap::new()
    }
  }

  fn add_routes (&mut self, path: &str, handler: Handler) {
    self.routes.insert(path.to_string(), handler);
  }

  fn handle_request (&self, path: &str, stream: &mut TcpStream) {
    match self.routes.get(path) {
      Some(handler) => {
        handler(stream)
      }
      Err(e) => {
        eprintln!("ERROR 404: {}", e);
      }
    }
  }
}