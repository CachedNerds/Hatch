use generators::Generator;
use assets::ProjectAsset;
use project::Project;

pub struct Tup<'tup> {
  project: &'tup Project
}

impl<'tup> Tup<'tup> {
  pub fn new(project :&Project) -> Tup {
    Tup { project }
  }
  pub fn boxed(project :&Project) -> Box<Tup> {
    Box::new(Tup::new(project))
  }
}

impl<'generator> Generator<'generator> for Tup<'generator> {
  fn asses(&self) -> Vec<ProjectAsset> {
    unimplemented!()
  }
}