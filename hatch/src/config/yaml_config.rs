use std::fs;
use std::io::Read;
use yaml_rust::{ YamlLoader, Yaml };
use std::path::{ Path, PathBuf };
use hatch_error::{ HatchResult, ResultExt };
use config::{ ConfigLoader };
use visitor::config_visitor::YamlConfigVisitor;

#[derive(Debug)]
pub struct YamlConfigLoader {
  file_path: PathBuf,
}

impl YamlConfigLoader {
  pub fn new(file_path: &Path) -> YamlConfigLoader {
    YamlConfigLoader { file_path: file_path.to_path_buf() }
  }
  
  pub fn read_yaml_file(&self, file_name: &str) -> HatchResult<Vec<Yaml>> {
    let file_with_path = self.file_path.join(file_name);
    let mut file = fs::File::open(&file_with_path).with_context(|_| {
      format!("failed to open file: `{}`", &file_with_path.display())
    })?;

    let mut contents = String::new();
    file.read_to_string(&mut contents).with_context(|_| {
      format!("failed to read contents of: `{}`", &file_with_path.display())
    })?;

    let res = YamlLoader::load_from_str(&contents).compat().with_context(|e| {
      format!("Parsing error: `{}`", e)
    })?;

    Ok(res)
  }
}

impl ConfigLoader for YamlConfigLoader {
  fn accept(&self, visitor: Box<&YamlConfigVisitor>) {
    visitor.visit_yaml_config(Box::new(self));
  }
}
