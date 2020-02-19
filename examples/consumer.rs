extern crate mq_rs;
extern crate serde_json;

use serde_json::{Value, json};

use mq_rs::mq::RMQ;
use mq_rs::MQ;
use mq_rs::Result;

fn main() {
  let mut mq = RMQ::new("test_module").unwrap();
  let body = json!({"hello": "from rust"});

  let listener = Box::new(move |v: Value| -> Result {
    println!("Received event! {}", v);
    Result::Ok
  });

  mq.bind("event.a", listener);
  mq.publish("event.a", body);

  mq.ready();
}