extern crate mq_rs;
extern crate serde_json;

use serde_json::{json};

use mq_rs::mq::RMQ;
use mq_rs::MQ;

fn main() {
  let mut mq = RMQ::new("test_module");
  loop {
    let body = json!({"hello": "from rust"});
    mq.publish("event.a", body);
    println!("published");
  }
}