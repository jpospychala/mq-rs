extern crate serde_json;
extern crate amqp;

use amqp::{Session, Basic, Channel, Table, protocol};
use amqp::QueueBuilder;
use amqp::protocol::basic;
use serde_json::{Value};
use std::collections::HashMap;
use std::rc::Rc;

use crate::{Listener, MQ};

pub struct RMQ {
    queue_name: String,
    channel: Channel,
    bindings: HashMap<String, Rc<Listener>>,
}

impl RMQ {
    pub fn new(name: &str) -> RMQ {
        let mut session = Session::open_url("amqp://localhost//").unwrap();

        let mut channel = session.open_channel(1).unwrap();
        let queue_name = name.to_string();
        let queue_builder = QueueBuilder::named(&queue_name).durable();
        let queue_declare = queue_builder.declare(&mut channel);

        RMQ{
            queue_name,
            channel,
            bindings: HashMap::new(),
        }
    }

    pub fn start_consuming(&mut self) {
        let closure_consumer = move |_chan: &mut Channel, deliver: basic::Deliver, headers: basic::BasicProperties, data: Vec<u8>|
        {
            println!("[closure] Deliver info: {:?}", deliver);
            println!("[closure] Content headers: {:?}", headers);
            println!("[closure] Content body: {:?}", data);
            println!("[function] Content body(as string): {:?}", String::from_utf8(data));

           /* self.channel.basic_publish(
                "amq.topic", 
                "event.a", 
                true, 
                false,
                protocol::basic::BasicProperties{ 
                    content_type: Some("text".to_string()), 
                    ..Default::default()
                }, 
                b"zupa zupa zupa".to_vec()
            ).unwrap();*/
//        channel.basic_ack(deliver.delivery_tag, false).unwrap();
        };
        let consumer_name = self.channel.basic_consume(closure_consumer, &self.queue_name, &"".to_string(), false, false, false, false, Table::new());
        println!("Starting consumer {:?}", consumer_name);

        self.channel.start_consuming();
    }
}

impl MQ for RMQ {
    fn bind(&mut self, routing_key: &str, cb: Rc<Listener>) {
        self.bindings.insert(routing_key.to_string(), cb);
        let qb = self.channel.queue_bind(&self.queue_name, &"amq.topic".to_string(), &routing_key.to_string(), false, Table::new());
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
}


#[cfg(test)]
mod tests {

    use super::*;
    use serde_json::*;

    #[test]
    fn finds_published_event() {
        let mq = RMQ::new("test_queue");
        let body = json!({"hello": "from rust"});

        let listener = move |v: &Value| -> crate::Result {
            crate::Result::Ok
        };
        
        //mq.bind("event.a", Rc::new(listener));
        // mq.publish("event.a", body);

        //mq.start_consuming();
    }
}
