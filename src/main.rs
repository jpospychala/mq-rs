use mq_rs::mq::RMQ;
use mq_rs::{MQ, Result};
use std::rc::Rc;
use serde_json::*;

fn main() {
  let mut mq = RMQ::new("module1");
  let body = json!({"hello": "from rust"});

  let listener = move |v: &Value| -> Result {
    Result::Ok
  };
  
  println!("bind");
  mq.bind("event.a", Rc::new(listener));
  println!("publish");
  mq.publish("event.a", body);

  mq.start_consuming();
}