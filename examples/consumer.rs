extern crate mq_rs;
extern crate serde_json;

use std::rc::Rc;
use serde_json::{Value, json};

use mq_rs::mq::RMQ;
use mq_rs::MQ;
use mq_rs::Result;

fn main() {
  let mut mq = RMQ::new("test_module");
  let body = json!({"hello": "from rust"});

  let listener = move |v: &Value| -> Result {
    println!("Received event!");
    Result::Ok
  };
  
  mq.bind("event.a", Rc::new(listener));
  mq.publish("event.a", body);

  mq.ready();
}