use visitor::config_visitor::YamlConfigVisitor;
use config::yaml_config::YamlConfigLoader;

struct HatchFileConfigVisitor;

impl HatchFileConfigVisitor {
  pub fn new() -> HatchFileConfigVisitor {
    HatchFileConfigVisitor {}
  }
}

impl YamlConfigVisitor for HatchFileConfigVisitor {
  fn visit_yaml_config(&self, config: Box<&YamlConfigLoader>) {
    let res = config.read_yaml_file("Hatch.yml");
    println!("{:?}", res);
  }
}
