mod tree;

#[cfg(test)]
mod tests {
  use super::tree::node_types::dir::Dir;
  use super::tree::node_types::file::File;

  use super::tree::node_types::Node;

    #[test]
    fn it_works() {
      let directory_node = Dir::new("hatch");
      let file_node = File::new("README.md");

      let mut root = Node::Branch(directory_node);
    }
}
