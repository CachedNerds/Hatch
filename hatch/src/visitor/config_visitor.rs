use config::yaml_config::YamlConfigLoader;

pub trait YamlConfigVisitor {
  fn visit_yaml_config(&self, config: Box<&YamlConfigLoader>);
}
