#![allow(dead_code)]
/// Route struct
///
pub struct Route {
  prefix: String,

}

impl Route {
  pub fn new () -> Route {
    Route {
      prefix: String::from(""),
    }
  }

  pub fn prefix (&mut self, prefix: &str) -> &mut Route {
    self.prefix.push_str(prefix);
    self
  }
}

