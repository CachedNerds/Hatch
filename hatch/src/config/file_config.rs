use config::{ ConfigLoader, ConfigFileLoader };
use std::collections::HashMap;
use std::path::{ Path, PathBuf };

pub struct FileConfigLoader {
  file_path: PathBuf
}

impl FileConfigLoader {
  pub fn new(file_path: &Path) -> FileConfigLoader {
    FileConfigLoader { file_path: file_path.to_path_buf() }
  }
}

impl ConfigLoader for FileConfigLoader {
  fn getConfig(&self, file: &str) -> HashMap<String, String> {
    self.getConfigImpl(&self.file_path, file)
  }
}

impl ConfigFileLoader for FileConfigLoader {
  fn getConfigImpl(&self, path: &Path, file: &str) -> HashMap<String, String> {
    // parse config file at path.join(file) into hashmap
    HashMap::new()
  }
}
