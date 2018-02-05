use config::{ ConfigLoader, ConfigFileLoader };
use std::collections::HashMap;
use std::path::Path;

pub struct FileConfigLoader {
  file_path: Path
}

impl FileConfigLoader {
  pub fn new(file_path: Path) -> FileConfigLoader {
    FileConfigLoader { file_path }
  }
}

impl Config for FileConfigLoader {
  pub fn getConfig(&self, file: String) -> HashMap<String, String> {
    self.getConfigImpl(self.file_path, file)
  }
}

impl ConfigFileLoader for FileConfigLoader {
  fn getConfigImpl(&self, path: String, file: String) -> HashMap<String, String> {
    // parse config file at path.join(file) into hashmap
  }
}
