extern crate serde_json;

use serde_json::{Value};
use std::collections::HashMap;

use crate::{Listener, MQ, FakeMQ};

#[derive(Default)]
pub struct MemMQ {
  bindings: HashMap<String, Listener>,
  published_events: Vec<(String, Value)>,
}

impl MQ for MemMQ {
  fn bind(&mut self, routing_key: &str, cb: Listener) {
    self.bindings.insert(routing_key.to_string(), cb);
  }

  fn publish(&mut self, routing_key: &str, body: Value) {
    self.published_events.push((routing_key.to_string(), body))
  }

  fn ready(&mut self) {}
}

impl FakeMQ for MemMQ {
  fn having_incoming(&self, routing_key: &str, body: Value) {
    if let Some(binding) = self.bindings.get(routing_key) {
      binding(body);
    }
  }

  fn has_published(&mut self, routing_key: &str) -> Vec<&Value> {
    self.published_events.iter()
      .filter(|(rk,_)| routing_key == rk)
      .map(|(_,val)| val)
      .collect()
  }

  fn reset(&mut self) {
    self.published_events.clear();
  }
}

#[cfg(test)]
mod tests {

  use std::cell::RefCell;
  use super::*;
  use serde_json::*;

  #[test]
  fn finds_published_event() {
    let mut mq = MemMQ::new();
    let body = json!(null);
    mq.publish("event.a", body);

    assert_eq!(mq.has_published("event.a"), vec![&json!(null)])
  }

  #[test]
  fn bind_hooks_listener_to_call_with_event() {
    let mut mq = MemMQ::new();
    let called: RefCell<Option<Value>> = RefCell::new(None);
    let called_in_cb = called.clone();
    let listener: Listener = Box::new(move |v: Value| -> crate::Result {
      let cloned = v.clone();
      called_in_cb.replace(Some(cloned));
      crate::Result::Ok
    });
    mq.bind("event.a", listener);
    mq.having_incoming("event.a", json!({"prop": true}));
    assert_eq!(*(called.borrow()), Some(json!({"prop": true})));
  }
}
