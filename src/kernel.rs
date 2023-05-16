use std::any::{Any, type_name, TypeId};
use std::collections::HashMap;
use std::fmt::Pointer;
use std::iter::Flatten;
use std::ops::Deref;
use std::sync::{Arc, Mutex};
use crate::server::Server;

pub trait Service: AsAny + AsAnyMut {
  fn start(&self);
}

pub trait AsAnyMut {
  fn as_any_mut(&mut self) -> &mut dyn Any;
}

impl<T: Any> AsAnyMut for T {
  fn as_any_mut(&mut self) -> &mut dyn Any {
    self
  }
}

pub trait AsAny {
  fn as_any(&self) -> &dyn Any;
}

impl<T: Any> AsAny for T {
  fn as_any(&self) -> &dyn Any {
    self
  }
}

pub struct Kernel {
  services: HashMap<TypeId, Arc<dyn Service>>,
}

impl Kernel {
  pub fn new() -> Self {
    Kernel {
      services: HashMap::new(),
    }
  }

  pub fn instance () -> Arc<Mutex<Self>> {
    static mut INSTANCE: Option<Arc<Mutex<Kernel>>> = None;
    static ONCE: std::sync::Once = std::sync::Once::new();

    unsafe {
      ONCE.call_once(|| {
        let kernel = Kernel::new();
        INSTANCE = Some(Arc::new(Mutex::new(kernel)));
      });

      INSTANCE.as_ref().unwrap().clone()
    }
  }

  pub fn register_service<T>(&mut self, service: Arc<T>)
    where
      T: 'static + Service + Send + Sync,
  {
    self.services.insert(TypeId::of::<T>(), service);
  }

  pub fn get_service<T>(&self) -> Option<Arc<T>>
    where
      T: 'static + Service,
  {
    self.services.get(&TypeId::of::<T>())
      .map(|service| {
        service.as_any()
          .downcast::<T>()
          .unwrap().clone()
      })
  }

  pub fn get_service_mut<T: 'static>(&mut self) -> Option<&mut T> {
    if let Some(service) = self.services.get_mut(&TypeId::of::<T>()) {
      service.as_mut().as_any_mut().downcast_mut::<T>()
    } else {
      None
    }
  }

  pub fn boot (&mut self) {
    for (type_id, service) in &mut self.services {
      service.start();

    }
  }
}