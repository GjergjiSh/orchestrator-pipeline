//TODO: Remove SharedData dependency and send byte vec or String
//TODO: Check if multiple pubs is even worth it

pub mod cfg;

use crate::{prelude::SharedData, shared_data::specification::Motors};
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
    sub_functions: HashMap<String, Box<dyn Fn(&Self, &Vec<u8>, &mut SharedData)>>,
    pub_functions: HashMap<String, Box<dyn Fn(&Self, &Socket, &SharedData)>>,
}

impl Connection {
    pub fn new(config: &ConnectionConfig) -> Self {
        let publisher_count = config.publishers().len();
        let subscriber_count = config.subscribers().len();

        let mut pub_functions: HashMap<String, Box<dyn Fn(&Self, &Socket, &SharedData)>> =
            HashMap::with_capacity(publisher_count);

        let mut sub_functions: HashMap<String, Box<dyn Fn(&Self, &Vec<u8>, &mut SharedData)>> =
            HashMap::with_capacity(subscriber_count);

        //This can be instrumented
        pub_functions.insert(
            "Motors".to_string(),
            Box::new(|conn, socket, shared_data| conn.publish_motors(socket, shared_data)),
        );

        //This can be instrumented
        sub_functions.insert(
            "Motors".to_string(),
            Box::new(|conn, message, shared_data| conn.recv_motors(message, shared_data)),
        );

        let mut connection = Connection {
            context: zmq::Context::new(),
            host: config.host(),
            port: config.port(),
            publishers: HashMap::with_capacity(publisher_count),
            subscribers: HashMap::with_capacity(subscriber_count),
            pub_functions: pub_functions,
            sub_functions,
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
    fn recv_motors(&self, message: &Vec<u8>, shared_data: &mut SharedData) {
        let new_motor_values = Motors::parse_from_bytes(message).unwrap();
        let motors = shared_data.motors.as_mut().unwrap();
        *motors = new_motor_values;
        dbg!(motors);
    }

    //TODO: Error handling
    pub fn send(&self, shared_data: &SharedData) {
        for (topic, socket) in &self.publishers {
            let pub_function = self.pub_functions.get(topic).unwrap();
            pub_function(&self, socket, shared_data);
        }
    }

    //TODO: Config for blocking vs non-blocking
    //TODO: Error handling
    //TODO: Check if having a hashmap of subs even makes sense
    pub fn recv(&self, shared_data: &mut SharedData) {
        for (_, socket) in  &self.subscribers {
            if let Ok(message) = socket.recv_bytes(zmq::DONTWAIT) {
                let topic = String::from_utf8_lossy(&message).into_owned();
                if let Ok(data) = socket.recv_bytes(zmq::DONTWAIT) {
                    let sub_function = self.sub_functions.get(&topic).unwrap();
                    sub_function(&self, &data, shared_data);
                }
            }
        }
    }

}