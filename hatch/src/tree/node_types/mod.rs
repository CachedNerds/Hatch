pub mod file;
pub mod dir;

pub enum Node {
  Leaf(file::File),
  Branch(dir::Dir),
}
