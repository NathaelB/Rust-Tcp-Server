use std::any::{Any, TypeId};
use std::collections::HashMap;

pub struct ServiceRegistry {
  pub(crate) services: HashMap<TypeId, Box<dyn Any>>
}

impl ServiceRegistry {
  pub fn new () -> Self {
    ServiceRegistry {
      services: HashMap::new()
    }
  }

  pub fn register<T: 'static> (&mut self, service: T) {
    self.services.insert(TypeId::of::<T>(), Box::new(service));
  }

  pub fn get<T: 'static> (&self) -> Option<&T> {
    self.services
      .get(&TypeId::of::<T>())
      .and_then(|service| service.downcast_ref::<T>())
  }

  pub fn get_mut<T: 'static> (&mut self) -> Option<&mut T> {
    self.services
      .get_mut(&TypeId::of::<T>())
      .and_then(|service| service.downcast_mut::<T>())
  }


}