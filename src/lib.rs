extern crate serde_json;

use serde_json::{Value};
use std::rc::Rc;

pub mod memmq;

pub enum Result {
    Ok,
    Err,
}

pub type Listener = dyn Fn(&Value) -> Result;

pub trait MQ {
    fn bind(&mut self, routing_key: &str, cb: Rc<Listener>);
    fn publish(&mut self, routing_key: &str, body: Value);
}

pub trait FakeMQ {
    fn having_incoming(&self, routing_key: &str, body: Value);
    fn has_published(&mut self, routing_key: &str) -> Vec<&Value>;
    fn reset(&mut self);
}
