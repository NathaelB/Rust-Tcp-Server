mod server;
mod router;

use std::{io, thread};
use server::Server;
use router::Router;

fn main () {
  let server = Server::new("127.0.0.1:3333");
  let mut router = Router::new();

  router.add_route("/hello", |stream| {
    stream.write(b"Hello, World!").unwrap();
  });

  server.run();
}