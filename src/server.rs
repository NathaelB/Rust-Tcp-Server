use std::error::Error;
use std::io::{ErrorKind, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::sync::mpsc::channel;
use std::thread;
use crate::router::Router;

pub(crate) struct Server {
  pub listener: TcpListener,
  pub router: Router,
  pub max_connections: usize,
  pub current_connections: Arc<Mutex<usize>>
}

impl Server {
  pub fn new (addr: &str, max_connections: usize) -> Result<Self, Box<dyn Error>> {
    let listener = TcpListener::bind(addr).unwrap();
    let router = Router::new();

    Ok(Server {
      listener,
      router,
      max_connections,
      current_connections: Arc::new(Mutex::new(0)),
    })
  }

  pub fn run(&self) {
    println!("Server TCP running in port : 3333");

    let (tx, _rx) = channel::<()>();

    for stream in self.listener.incoming() {
      let tx = tx.clone();
      let current_connections = self.current_connections.clone();

      let mut current_connections_guard = current_connections.lock().unwrap();
      if *current_connections_guard >= self.max_connections {
        continue;
      }
      *current_connections_guard += 1;

      let stream = stream.unwrap();
      let router = self.router.clone();
      let tx1 = tx.clone();

      thread::spawn(move || {
        Self::handle_client(&router, stream);
        let _ = tx1.send(());
      });
    }
  }

  fn handle_client (router: &Router, mut stream: TcpStream) {
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
          let request_line = message.lines().next().unwrap_or("");
          let parts: Vec<&str> = request_line.split_whitespace().collect();
          if parts.len() > 1 {
            let path = parts[1];
            router.handle_request(path, &mut stream);
          } else {
            // Handle invalid request
          }

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