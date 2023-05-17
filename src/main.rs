mod server;
mod router;
mod database;

use std::io::Write;
use std::net::Shutdown;
use server::Server;
use crate::router::Router;

fn main () {
  let router = Router::new()
    .add_route("/hello", |stream| {
      stream.write(b"Hello, World!").unwrap();
      stream.flush().unwrap();
    }).unwrap();

  Server::new(("127.0.0.1", 3333), 10, router).run();
}