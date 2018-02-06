pub mod yaml_config;

use std::path::Path;
use hatch_error::HatchResult;
use visitor::config_visitor::YamlConfigVisitor;

pub trait ConfigLoader {
  fn accept(&self, visitor: Box<&YamlConfigVisitor>);
}
