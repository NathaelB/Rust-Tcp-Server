mod server;

use std::{io, thread};
use server::Server;

fn main () {
  let server = Server::new("127.0.0.1:3333");

  server.run();
}