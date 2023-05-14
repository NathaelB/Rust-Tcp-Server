mod server;
mod router;

use std::io::Write;
use std::net::Shutdown;
use server::Server;

fn main () {
  let server = Server::new("127.0.0.1:3333", 10);

  match server {
    Ok(mut server) => {
      server.router.add_route("/hello", |stream| {
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
      });

      server.run();
    }
    Err(e) => {
      println!("Error lors du démarrage: {}", e);
    }
  };
}