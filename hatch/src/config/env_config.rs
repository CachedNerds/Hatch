use config::{ ConfigLoader, ConfigFileLoader };
use std::collections::HashMap;
use std::path::Path;
use std::env;

pub struct EnvironmentVariableConfigLoader {
  var: Path
}

impl EnvironmentVariableConfigLoader {
  pub fn new(var: Path) -> EnvironmentVariableConfigLoader {
    EnvironmentVariableConfigLoader { var }
  }
}

impl ConfigLoader for EnvironmentVariableConfigLoader {
  pub fn getConfig(&self, file: String) -> HashMap<String, String> {
    self.getConfigImpl(env::var(self.var), file)
  }
}

impl ConfigFileLoader for EnvironmentVariableConfigLoader {
  fn getConfigImpl(&self, path: String, file: String) -> HashMap<String, String> {
    // parse config file at path.join(file) into hashmap
    HashMap::new()
  }
}
