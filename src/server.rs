use std::any::Any;
use std::error::Error;
use std::io::{ErrorKind, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::sync::mpsc::channel;
use std::thread;
use crate::kernel::{Kernel, Service};
use crate::router::Router;

pub(crate) struct Server {
  pub listener: Arc<TcpListener>,
  pub router: Arc<Mutex<Router>>,
  pub max_connections: usize,
  pub current_connections: Arc<Mutex<usize>>
}

impl Server {
  pub fn new (addr: &str, max_connections: usize, router: &Router) -> Result<Self, Box<dyn Error>> {
    let listener = Arc::new(TcpListener::bind(addr)?);

    Ok(Server {
      listener,
      router: Arc::new(Mutex::new(router.clone())),
      max_connections,
      current_connections: Arc::new(Mutex::new(0)),
    })
  }

  pub fn run(&self) {
    println!("Server TCP running in port : 3333");

    let (tx, _rx) = channel::<()>();
    let listener = self.listener.unwrap();
    for stream in listener.incoming() {
      let tx = tx.clone();
      let current_connections = self.current_connections.clone();

      let mut current_connections_guard = current_connections.lock().unwrap();
      if *current_connections_guard >= self.max_connections {
        continue;
      }
      *current_connections_guard += 1;

      let router = self.router.clone();
      let tx1 = tx.clone();

      thread::spawn(move || {
        let mut stream = stream.unwrap();
        Self::handle_client(router, stream);
        let _ = tx1.send(());
      });
    }
  }

  fn handle_client (router: Arc<Mutex<Router>>, mut stream: TcpStream) {
    let mut buffer = [0; 512];
    let client_addr = stream.peer_addr().unwrap();

    loop {
      match stream.read(&mut buffer) {
        Ok(bytes_read) => {
          if bytes_read == 0 {
            println!("Client {} disconnected", client_addr);
            break;
          }
          let message = String::from_utf8_lossy(&buffer[..bytes_read]);
          println!("Received message from client {}: {}", client_addr, message);

          // Parse the request and extract the path
          let mut router_guard = router.lock().unwrap();
          router_guard.handle_request(&mut stream, &buffer[..bytes_read]);
          drop(router_guard);
        }
        Err(e) => match e.kind() {
          ErrorKind::WouldBlock => (),
          _ => {
            eprintln!("Error reading from client {}: {}", client_addr, e);
            let error_message = format!("Error: {}", e);
            stream.write(error_message.as_bytes()).unwrap_or_default();
            break;
          }
        },
      }
    }
  }
}

impl Service for Server {
  fn start(&self) {
    let mut server = self;

    /*server.router.add_route("/hello", |stream| {
      stream.write(b"Hello, World!").unwrap();
      stream.flush().unwrap();
    });

    server.router.add_route("/algebre", |stream| {
      stream.write(b"L'algebre ton pire cauchemar!").unwrap();
      stream.flush().unwrap();
    });

    server.router.add_route("/end", |stream| {
      println!("DISCONECTED CONNEXION");
      stream.shutdown(Shutdown::Both).unwrap();
    });*/
    server.run();
  }
}

impl Clone for Server {
  fn clone(&self) -> Self {
    Server {
      listener: self.listener.clone(),
      router: self.router.clone(),
      max_connections: self.max_connections,
      current_connections: self.current_connections.clone(),
    }
  }
}