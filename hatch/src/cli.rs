use dtl::cli as cli_imp;
use project::{ ProjectKind };

pub struct Cli<'cli>(cli_imp::Cli<'cli>);

impl<'cli>Cli<'cli> {
  pub fn new() -> Cli<'cli> {
    Cli(cli_imp::Cli::new())
  }
  
  pub fn build_type(&mut self) -> ProjectKind {
    self.0.build_type()
  }

  pub fn name(&mut self) -> String {
    self.0.name()
  }

  pub fn path(&mut self) -> String {
    self.0.path()
  }
}
