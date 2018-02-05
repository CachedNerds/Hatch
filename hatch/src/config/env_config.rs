use config::{ ConfigLoader, ConfigFileLoader };
use std::collections::HashMap;
use std::path::{ Path, PathBuf };
use std::env;

pub struct EnvironmentVariableConfigLoader {
  var: String
}

impl EnvironmentVariableConfigLoader {
  pub fn new(var: &str) -> EnvironmentVariableConfigLoader {
    EnvironmentVariableConfigLoader { var: var.to_owned() }
  }
}

impl ConfigLoader for EnvironmentVariableConfigLoader {
  fn getConfig(&self, file: &str) -> HashMap<String, String> {
    if let Ok(ev) = env::var(self.var.as_ref() as &str) {
      self.getConfigImpl(&PathBuf::from(ev), file)
    } else {
      panic!("better death tbd, operation failed");
    }
  }
}

impl ConfigFileLoader for EnvironmentVariableConfigLoader {
  fn getConfigImpl(&self, path: &Path, file: &str) -> HashMap<String, String> {
    // parse config file at path.join(file) into hashmap
    HashMap::new()
  }
}
