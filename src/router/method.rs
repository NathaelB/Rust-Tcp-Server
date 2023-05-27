use std::convert::Infallible;

pub struct MethodRouter<S = (), E = Infallible> {
  get: Option<Box<dyn Handler<S, E>>>,
  post: Option<Box<dyn Handler<S, E>>>,
  put: Option<Box<dyn Handler<S, E>>>,
  delete: Option<Box<dyn Handler<S, E>>>,
  patch: Option<Box<dyn Handler<S, E>>>,
  head: Option<Box<dyn Handler<S, E>>>,
  options: Option<Box<dyn Handler<S, E>>>,
}


