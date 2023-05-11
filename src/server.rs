use std::fmt::format;
use std::io::{ErrorKind, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::thread;
use std::sync::{Arc, mpsc, Mutex};

static THREAD_COUNTER: AtomicUsize = AtomicUsize::new(0);

pub(crate) struct Server {
  pub listener: TcpListener,
}

impl Server {
  pub fn new (addr: &str) -> Server {
    let listener = TcpListener::bind(addr).unwrap();

    Server { listener }
  }

  pub fn run(&self) {
    println!("Server TCP running in port : 3333");
    for stream in self.listener.incoming() {
      let stream = stream.unwrap();
      thread::spawn(move || Self::handle_client(stream));
    }
  }

  fn handle_client (mut stream: TcpStream) {
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

          let response = "Message reçu!";
          stream.write(response.as_bytes()).unwrap();
          stream.flush().unwrap();
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