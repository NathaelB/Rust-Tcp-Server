mod server;
mod router;
mod database;
mod http;

use std::io::Write;
use std::net::Shutdown;
use server::Server;
use crate::http::builder::HttpResponse;
use crate::router::Router;

fn hello_world (stream: &mut std::net::TcpStream) {
  let response = HttpResponse::new(200)
    .header("Content-Type", "text/html")
    .body("<h1>Hello World</h1>".to_string())
    .build();

  stream.write(&response).unwrap();
  stream.flush().unwrap();
  stream.shutdown(Shutdown::Both).unwrap();
}

fn main () {
  Server::new(10)
    .bind(("127.0.0.1", 8888))
    .service("/hello", hello_world)
    .run();
}