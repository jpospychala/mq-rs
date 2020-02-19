extern crate serde_json;

use serde_json::{Value};

pub mod memmq;
pub mod mq;

pub enum Result {
  Ok,
  Err,
}

pub type Listener = Box<dyn FnMut(Value) -> Result + Send + Sync>;

pub trait MQ {
  fn bind(&mut self, routing_key: &str, cb: Listener);
  fn publish(&mut self, routing_key: &str, body: Value);
  fn ready(&mut self);
}

pub trait FakeMQ {
  fn having_incoming(&mut self, routing_key: &str, body: Value);
  fn has_published(&mut self, routing_key: &str) -> Vec<&Value>;
  fn reset(&mut self);
}
