mod server;
mod router;
mod database;
mod kernel;

use std::io::Write;
use std::net::{Shutdown, TcpStream};
use std::sync::{Arc, Mutex};
use server::Server;
use crate::kernel::Kernel;
use crate::router::Router;

fn main() {
  let mut kernel = Kernel::new();
  let mut router_service = Router::new();

  println!("router: {:?}", router_service);

  let router_ptr = std::ptr::addr_of!(router_service);
  println!("Pointer de router_service in main: {:p}", router_ptr);

  kernel.register_service(router_service.clone());

  let tcp_service = Server::new("127.0.0.1:3333", 10, &router_service).expect("Error TCP Server");

  kernel.register_service(tcp_service);


  // Récupération de l'instance de Router enregistrée dans le kernel
  let router = kernel.get_service_mut::<Router>().unwrap();

  // Enregistrement de la route avec l'instance de Router enregistrée dans le kernel
  router.add_route_fn("/test", |stream: &mut TcpStream| {
    stream.write(b"Hello, World!").unwrap();
    stream.flush().unwrap();
  });

  if std::ptr::eq(router, &router_service) {
    println!("Les deux instances sont les mêmes");
  } else {
    println!("Les deux instances sont différentes");
  }
  kernel.boot();
}