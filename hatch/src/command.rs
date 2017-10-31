use dtl::tup::{ Assets };

pub trait Command<T: Assets> {
  fn execute(&self) -> T;
}
