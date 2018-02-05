use config::{ ConfigLoader, ConfigFileLoader };
use std::env;
use std::collections::HashMap;

struct EnvironmentVariableConfigLoader;

impl Config for EnvironmentVariableConfigLoader {
  pub fn getConfig(&self, file: String) -> HashMap<String, String> {
    self.getConfigImpl(env::var("HATCH_CONFIG_PATH"), file)
  }
}

impl ConfigFileLoader for EnvironmentVariableConfigLoader {
  pub fn getConfigImpl(&self, path: String, file: String) -> HashMap<String, String> {
    // parse config file at path.join(file) into hashmap
  }  
}
