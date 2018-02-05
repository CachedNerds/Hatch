pub mod env_config;
pub mod file_config;

use std::collections::HashMap;
use std::path::Path;

pub trait ConfigLoader {
  fn getConfig(&self, source: String) -> HashMap<String, String>;
}

pub trait ConfigFileLoader : ConfigLoader {
  fn getConfigImpl(&self, path: Path, file: String) -> HashMap<String, String>;
}
