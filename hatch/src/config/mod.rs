pub mod env_config;

use std::collections::HashMap;

pub trait ConfigLoader {
  fn getConfig(&self, source: String) -> HashMap<String, String>;
}

pub trait ConfigFileLoader : ConfigLoader {
  fn getConfigImpl(&self, path: String, file: String) -> HashMap<String, String>;
}
