use std::any::Any;
use std::collections::HashMap;
use std::fs::read;
use std::io::{BufRead, BufReader, Read, Write};
use std::net::TcpStream;
use std::sync::{Arc, Mutex};
use crate::kernel::Service;

type Fn = dyn FnMut(&mut TcpStream) + Send + Sync + 'static;
type Handler = Arc<Mutex<Fn>>;

enum Route {
  Static(String),
  Parameter(String)
}

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

  pub fn add_route_fn (&mut self, url: &str, f: impl FnMut(&mut TcpStream) + Send + Sync + 'static) {
    println!("J'ai enregistré la route {}", url);
    self.routes.insert(url.to_string(), Arc::new(Mutex::new(f)));
  }

  pub fn handle_request(&mut self, stream: &mut TcpStream, buffer: &[u8]) {
    println!("Je rentre dans la fonction handle_request");
    let request_line = String::from_utf8_lossy(buffer);

    // Parse the request and extract the path
    let request_parts: Vec<&str> = request_line.split_whitespace().collect();
    let method = request_parts.get(0).map(|s| *s);
    let url = request_parts.get(1).map(|s| *s);

    println!("TEST 1");

    // ... autres traitements de la requête ...
    println!("URL: {}", url.unwrap_or(""));
    println!("Nombre de routes: {}", self.routes.len());
    if let Some("GET") = method {
      if let Some(handler) = self.routes.get_mut(url.unwrap_or("")) {
        let mut handler = handler.lock().unwrap();
        // Exécuter le gestionnaire associé à la route

        println!("TEST 2");
        handler(stream);
      } else {
        // Route non trouvée, répondre avec une erreur 404
        let response = "HTTP/1.1 404 Not Found\r\n\r\n404 Not Found";
        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
      }
    }

    // Créer lines pour traiter le reste de la requête
    let reader = BufReader::new(stream);
    let mut lines = reader.lines();
    // ...
  }
}

impl Service for Router {
  fn start(&self) {
    println!("Router Service UP : {}", &self.routes.len());
  }
}

/*impl Clone for Router {
  fn clone(&self) -> Self {
    Self {
      routes: self.routes.clone()
    }
  }
}*/

impl Clone for Router {
  fn clone(&self) -> Self {
    let mut new_routes = HashMap::new();

    for (key, value) in &self.routes {
      new_routes.insert(key.clone(), Arc::clone(value));
    }
    Self {
      routes: new_routes
    }
  }
}


