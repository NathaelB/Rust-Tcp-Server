mod server;
mod router;
mod database;
mod http;

use std::io::Write;
use std::net::Shutdown;
use server::Server;
use crate::http::builder::HttpResponse;
use crate::router::Router;

fn main () {
  let router = Router::new()
    .add_route("/hello", |stream| {
      let response = HttpResponse::new(200)
        .header("Content-Type", "text/plain")
        .body(b"Hello, World!")
        .build();

      stream.write(&response).unwrap();
      stream.flush().unwrap();
    }).unwrap();

  Server::new(("127.0.0.1", 3333), 10, router).run();
}