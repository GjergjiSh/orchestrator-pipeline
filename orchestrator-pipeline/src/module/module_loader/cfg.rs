use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize, Clone)]
pub struct IndividualModuleConfig {
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

        let file = std::fs::File::open(path)
            .unwrap_or_else(|e| panic!(
                "Failed to read module configuration file. Error: {}", e
            ));

        let config: ModuleConfig = serde_yaml::from_reader(&file)
            .unwrap_or_else(|e| panic!(
                "Failed to parse module configuration content. Error: {}", e
            ));

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