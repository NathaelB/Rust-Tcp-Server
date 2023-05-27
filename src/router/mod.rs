mod route;

use std::collections::HashMap;
use std::io::{Error, Write};
use std::net::{Shutdown, TcpStream};
use crate::http::builder::HttpResponse;

pub type Handler = fn(&mut TcpStream);


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

  pub fn add_route (&mut self, path: &str, handler: Handler) -> Result<&mut Router, Error> {
    self.routes.insert(path.to_string(), handler);

    Ok(self)
  }

  pub fn get_routes (&self) -> &HashMap<String, Handler> {
    &self.routes
  }

  pub fn handle_request(&self, path: &str, stream: &mut TcpStream) {

    match self.routes.get(path) {
      Some(handler) => {
        handler(stream)
      }
      None => {
        println!("ERROR 404");

        let response = HttpResponse::new(404)
          .header("Content-Type", "text/html")
          .body("<h1>404</h1>".to_string())
          .build();

        stream.write(&response).unwrap();
        stream.flush().unwrap();
        stream.shutdown(Shutdown::Both).unwrap();
      }
    }
  }
}