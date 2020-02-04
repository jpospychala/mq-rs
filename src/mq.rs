extern crate serde_json;
extern crate amqp;

use amqp::{Session, Basic, Channel, Table, protocol};
use amqp::QueueBuilder;
use amqp::protocol::basic;
use amqp::TableEntry;
use serde_json::{Value};
use std::collections::HashMap;
use std::rc::Rc;

use crate::{Listener, MQ};

pub struct RMQ {
  module_name: String,
  channel: Channel,
  bindings: HashMap<String, Rc<Listener>>,
}

impl RMQ {
  pub fn new(name: &str) -> RMQ {
    let mut session = Session::open_url("amqp://localhost//").unwrap();

    let channel = session.open_channel(1).unwrap();
    let module_name = name.to_string();

    RMQ{
      module_name,
      channel,
      bindings: HashMap::new(),
    }
  }
}

impl MQ for RMQ {
  fn bind(&mut self, routing_key: &str, cb: Rc<Listener>) {
    self.bindings.insert(routing_key.to_string(), cb);
    let events_queue = format!("{}:events", self.module_name);
    let _qb = self.channel.queue_bind(events_queue, "amq.topic".to_string(), routing_key.to_string(), false, Table::new());
  }

  fn publish(&mut self, routing_key: &str, body: Value) {
    self.channel.basic_publish(
      "amq.topic",
      routing_key,
      true,
      false,
      protocol::basic::BasicProperties{
        content_type: Some("text".to_string()),
        ..Default::default()
      },
      serde_json::to_vec(&body).unwrap()
    ).unwrap();
  }

  fn ready(&mut self) {
    let module_name = self.module_name.clone();
    let closure_consumer = move |_chan: &mut Channel, _deliver: basic::Deliver, headers: basic::BasicProperties, data: Vec<u8>|
    {
      let body = String::from_utf8(data).unwrap();
      if let Some(headers) = headers.headers {
        let table_entry = headers.get("reply_to").unwrap();
        match table_entry {
          TableEntry::LongString(s) => {
            if body == "PING" {
              _chan.basic_publish(
                "direct",
                s,
                true,
                false,
                protocol::basic::BasicProperties{
                  content_type: Some("text".to_string()),
                  ..Default::default()
                },
                module_name.as_bytes().to_vec()
              ).unwrap();
            } else {
              // TODO query response
            }
          }
          _ => {}
        }
      }
      println!("Consumed");
    };

    let queue_builder = QueueBuilder::named(self.module_name.to_string()).auto_delete().exclusive();
    let _queue_declare = queue_builder.declare(&mut self.channel);
    let cons1 = self.channel.basic_consume(closure_consumer, &self.module_name, &"".to_string(), false, true, true, false, Table::new());

    let event_consumer = move |_chan: &mut Channel, deliver: basic::Deliver, headers: basic::BasicProperties, data: Vec<u8>|
    {
      let v: Value = serde_json::from_slice(&data[..]).unwrap();
      let bindings_mut = &bindings_clone;
      if let Some(binding) = bindings_mut.get("") { // TODO how to steal self into closure?
        // binding(&v);
      }
      // println!("[closure] Deliver info: {:?}", deliver);
      println!("[closure] Content headers: {:?}", headers);
      // println!("[closure] Content body: {:?}", data);
      // println!("[function] Content body(as string): {:?}", String::from_utf8(data));

      _chan.basic_ack(deliver.delivery_tag, false).unwrap();
    };
    let events_queue = format!("{}:events", self.module_name);
    let queue_builder = QueueBuilder::named(events_queue.to_string()).durable();
    let _queue_declare = queue_builder.declare(&mut self.channel);
    let cons2 = self.channel.basic_consume(event_consumer, events_queue, "".to_string(), false, false, false, false, Table::new());


    println!("Starting consumer {:?} {:?}", cons1, cons2);

    // self.channel.basic_prefetch(1).ok().expect("Failed to prefetch"); // TODO parametrize prefetch
    self.channel.start_consuming();
  }
}


#[cfg(test)]
mod tests {

  use super::*;
  use serde_json::*;

  #[test]
  fn finds_published_event() {
    let mut mq = RMQ::new("test_module");
    let body = json!({"hello": "from rust"});

    let listener = move |_v: &Value| -> crate::Result {
      println!("Received event!");
      crate::Result::Ok
    };

    mq.bind("event.a", Rc::new(listener));
    mq.publish("event.a", body);

    mq.ready();
  }

  #[test]
  fn responds_to_ping() {
  }

  #[test]
  fn receives_message() {
  }

  #[test]
  fn invokes_correct_listener_for_routing() {
  }


}
