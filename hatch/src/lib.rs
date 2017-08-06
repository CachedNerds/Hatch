mod data {
  pub enum Template {
    tupfile,
    makefile,
  }
}

mod node {
  pub struct file {
    contents: String,
    file_name: String,
  }

  pub struct dir {
    contents: Vec <NodeType>,
    dir_name: String,
  }

  impl file {
    pub fn new(file_name: &str) -> file {
      file {
        contents: "".to_string(),
        file_name: file_name.to_string()
      }
    }

    pub fn update_content(&mut self, data: super::data::Template) {
    }
  }

  impl dir {
    pub fn new(dir_name: &str) -> dir {
      dir {
        contents: Vec::new(),
        dir_name: dir_name.to_string(),
      }
    }
    
    pub fn add_file(&mut self, key: &str) {
      self.contents.push(NodeType::leaf(file::new(key)));
    }

    pub fn add_dir(&mut self, key: &str) {
      self.contents.push(NodeType::node(dir::new(key)));
    }
  }

  pub enum NodeType {
    leaf(file),
    node(dir),
  }
}

#[cfg(test)]
mod tests {
  use super::node::NodeType;
  use super::node::file;
  use super::node::dir;

    #[test]
    fn it_works() {
      let mut root = dir::new("project_root");

      root.add_file("README.md");
    }
}
