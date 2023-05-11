mod server;
mod router;

use std::io::Write;
use server::Server;

fn main () {
  let mut server = Server::new("127.0.0.1:3333");


  server.router.add_route("/hello", |stream| {
    stream.write(b"Hello, World!").unwrap();
    stream.flush().unwrap();
  });

  server.run();
}