use std::collections::hash_map::HashMap;
// use zmq::Context;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ConnectionConfig {
    host: String,
    port: String,
    publishers: Vec<String>,
    subscribers: Vec<String>,
}

impl ConnectionConfig {
    pub fn new(path: &std::path::Path) -> Self {
        if !path.exists() {
            panic!("Connection configuration not found at: {:?}", path)
        }

        let file = std::fs::File::open(path)
            .unwrap_or_else(|e| panic!(
                "Failed to read connection configuration file. Error: {}", e));

        let config: ConnectionConfig = serde_yaml::from_reader(file)
            .unwrap_or_else(|e| panic!(
                "Failed to parse connection configuration content. Error: {}", e));

        config
    }

    pub fn host(&self) -> String {
        self.host.clone()
    }

    pub fn port(&self) -> String {
        self.port.clone()
    }

    pub fn publishers(&self) -> &Vec<String> {
        &self.publishers
    }

    pub fn subscribers(&self) -> &Vec<String> {
        &self.subscribers
    }
}

pub struct Connection {
    // context: Context,
    // publishers: HashMap<String, zmq::Socket>,
    // subscribers: HashMap<String, zmq::Socket>,
    host: String,
    port: String,
}

impl Connection {
    pub fn new(config: &ConnectionConfig) -> Self {
        let mut connection = Connection {
            // context: zmq::Context::new(),
            // publishers: HashMap::new(),
            // subscribers: HashMap::new(),
            host: config.host(),
            port: config.port(),
        };

        // for topic in &config.pub_topics {
        //     let socket = connection.make_pub_socket(topic);
        //     connection.publishers.insert(topic.clone(), socket);
        // }

        // for topic in &config.sub_topics {
        //     let socket = connection.make_sub_socket(topic);
        //     connection.subscribers.insert(topic.clone(), socket);
        // }

        connection
    }

    /* fn make_pub_socket(&mut self, topic: &str) {
        let socket = self
            .context
            .socket(zmq::PUB)
            .unwrap_or_else(|e| panic!("Failed to create PUB socket: {}", e));

        socket
            .connect(&format!("tcp://{}:{}", self.host, self.port))
            .unwrap_or_else(|e| panic!("Failed to connect PUB socket: {}", e));

        self.publishers.insert(topic.to_string(), socket.clone());
    }

    fn make_sub_socket(&mut self, topic: &str)  {
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
        self.subscribers.insert(topic.to_string(), socket.clone());
    } */
}
