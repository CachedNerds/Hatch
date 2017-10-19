use cli::{ Cli };
use project::{ Project };
use asset::{ Asset };


pub struct Egg<'cli> {
  cli: Cli<'cli>,
  project: Option<Project>,
  asset: Vec<Asset>,
}

impl<'cli> Egg<'cli> {
  pub fn new() -> Egg<'cli> {
    Egg { cli: Cli::new(), project: None, asset: Vec::new() }
  }
}
