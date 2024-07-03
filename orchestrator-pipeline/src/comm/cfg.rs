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