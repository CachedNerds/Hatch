use dtl::egg as egg_imp;

pub struct Egg<'cli>(egg_imp::Egg<'cli>);

impl<'cli> Egg<'cli> {
  pub fn new() -> Egg<'cli> {
    Egg(egg_imp::Egg::new())
  }
}
