use assets::ProjectAsset;

pub mod tup;

pub trait Generator<'generator> {
  fn asses(&self) -> Vec<ProjectAsset>;
}