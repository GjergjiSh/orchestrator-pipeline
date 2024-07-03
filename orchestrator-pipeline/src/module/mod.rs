use crate::shared_data::specification::SharedData;
use serde::Deserialize;
use std::{collections::HashMap, io::Read};

#[derive(Deserialize, Clone)]
struct IndividualModuleConfig {
    name: String,
    parameters: HashMap<String, String>,
}

#[derive(Deserialize)]
pub struct ModuleConfig {
    module_root: String,
    modules: Vec<IndividualModuleConfig>,
}

impl ModuleConfig {
    pub fn new(path: &std::path::Path) -> Self {
        if !path.exists() {
            panic!("Module configuration not found at: {:?}", path)
        }

        let mut file = std::fs::File::open(path)
            .unwrap_or_else(|e| panic!("Failed to read configuration file. Error: {}", e));

        let mut config_content = String::new();
        file.read_to_string(&mut config_content);

        let config: ModuleConfig = serde_yaml::from_str(&config_content)
            .unwrap_or_else(|e| panic!("Failed to parse configuration content. Error: {}", e));

        config
    }

    pub fn modules(&self) -> HashMap<String, HashMap<String, String>> {
        self.modules
            .clone()
            .into_iter()
            .map(|module| (module.name, module.parameters))
            .collect()
    }

    pub fn module_root(&self) -> &String {
        &self.module_root
    }
}

pub trait Module {
    fn process(&self, shared_data: &SharedData);
}
