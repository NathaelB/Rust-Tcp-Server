pub struct BaseModel {
  pub primary_key: String,
  pub booted: bool,
  pub table: String,
}

pub trait Model<T> {
  fn create ();
  fn table ();
}
