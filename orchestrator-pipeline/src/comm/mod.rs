//TODO: Remove SharedData dependency and send byte vec or String
//TODO: Check if multiple pubs is even worth it

pub mod cfg;

use crate::prelude::SharedData;
use cfg::ConnectionConfig;
use protobuf::Message;
use std::collections::hash_map::HashMap;
use zmq::{Context, Socket};

pub struct Connection {
    context: Context,
    port: String,
    host: String,
    publishers: HashMap<String, zmq::Socket>,
    subscribers: HashMap<String, zmq::Socket>,
    pub_functions: HashMap<String, Box<dyn Fn(&Self, &Socket, &SharedData)>>,
}

impl Connection {
    pub fn new(config: &ConnectionConfig) -> Self {
        let publisher_count = config.publishers().len();
        let subscriber_count = config.subscribers().len();

        let mut pub_functions: HashMap<String, Box<dyn Fn(&Self, &Socket, &SharedData)>> =
            HashMap::with_capacity(publisher_count);

        pub_functions.insert(
            "Motors".to_string(),
            Box::new(|conn, socket, shared_data| conn.publish_motors(socket, shared_data)),
        );

        let mut connection = Connection {
            context: zmq::Context::new(),
            host: config.host(),
            port: config.port(),
            publishers: HashMap::with_capacity(publisher_count),
            subscribers: HashMap::with_capacity(subscriber_count),
            pub_functions: pub_functions,
        };

        for topic in config.publishers() {
            connection.make_pub_socket(topic);
        }

        for topic in config.subscribers() {
            connection.make_sub_socket(topic);
        }

        connection
    }

    fn make_pub_socket(&mut self, topic: &str) {
        if self.publishers.contains_key(topic) {
            panic!("Duplicate topic for publisher: {}", topic);
        }

        let socket = self
            .context
            .socket(zmq::PUB)
            .unwrap_or_else(|e| panic!("Failed to create PUB socket: {}", e));

        socket
            .bind(&format!("tcp://{}:{}", self.host, self.port))
            .unwrap_or_else(|e| panic!("Failed to connect PUB socket: {}", e));

        self.publishers.insert(topic.to_string(), socket);
    }

    fn make_sub_socket(&mut self, topic: &str) {
        if self.subscribers.contains_key(topic) {
            panic!("Duplicate topic for subscriber: {}", topic);
        }

        let socket = self
            .context
            .socket(zmq::SUB)
            .unwrap_or_else(|e| panic!("Failed to create SUB socket: {}", e));

        socket
            .connect(&format!("tcp://{}:{}", self.host, self.port))
            .unwrap_or_else(|e| panic!("Failed to connect SUB socket: {}", e));

        socket
            .set_subscribe(topic.as_bytes())
            .unwrap_or_else(|e| panic!("Failed to subscribe to topic '{}': {}", topic, e));

        self.subscribers.insert(topic.to_string(), socket);
    }

    //TODO: Error handling
    fn publish_motors(&self, socket: &Socket, shared_data: &SharedData) {
        let motors = shared_data.motors.as_ref().unwrap();
        let motors_ser = motors.write_to_bytes().unwrap();
        socket.send("Motors", zmq::SNDMORE).unwrap();
        socket.send(motors_ser, 0).unwrap();
    }

    //TODO: Error handling
    pub fn send(&self, shared_data: &SharedData) {
        for (topic, socket) in &self.publishers {
            let pub_function = self.pub_functions.get(topic).unwrap();
            pub_function(&self, socket, shared_data);
        }
    }
}