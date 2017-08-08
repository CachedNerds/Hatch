use super::Node;
use super::file::File;

pub struct Dir {
  contents: Vec <super::Node>,
  dir_name: String,
}

impl Dir {
  pub fn new(dir_name: &str) -> Dir {
    Dir {
      contents: Vec::new(),
      dir_name: dir_name.to_string(),
    }
  }

  pub fn add_file(&mut self, file_name: &str) {
    self.contents.push(Node::Leaf(File::new(file_name)));
  }

  pub fn add_dir(&mut self, dir_name: &str) {
    self.contents.push(Node::Branch(Dir::new(dir_name)));
  }
}
