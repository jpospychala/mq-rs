extern crate serde_json;

use serde_json::{Value};
use std::collections::HashMap;
use std::rc::Rc;

use crate::{Listener, MQ, FakeMQ};

pub struct MemMQ {
    bindings: HashMap<String, Rc<Listener>>,
    published_events: Vec<(String, Value)>,
}

impl MemMQ {
    pub fn new() -> MemMQ {
        MemMQ{
            bindings: HashMap::new(),
            published_events: vec![]
        }
    }
}

impl MQ for MemMQ {
    fn bind(&mut self, routing_key: &str, cb: Rc<Listener>) {
        self.bindings.insert(routing_key.to_string(), cb);
    }

    fn publish(&mut self, routing_key: &str, body: Value) {
        self.published_events.push((routing_key.to_string(), body))
    }
}

impl FakeMQ for MemMQ {
    fn having_incoming(&self, routing_key: &str, body: Value) {
        for binding in self.bindings.get(routing_key) {
            binding(&body);
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
        let called = Rc::new(RefCell::new(None));
        let called_in_cb = called.clone();
        let listener = move |v: &Value| -> crate::Result {
            called_in_cb.replace(Some(v.clone()));
            crate::Result::Ok
        };
        mq.bind("event.a", Rc::new(listener));
        mq.having_incoming("event.a", json!({"prop": true}));
        assert_eq!(*(called.borrow()), Some(json!({"prop": true})));
    }
}
