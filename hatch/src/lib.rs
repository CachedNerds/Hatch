mod node
{
  pub struct file
  {
    file_name: String,
  }

  pub struct dir
  {
    dir_name: String,
  }

  impl file
  {
    pub fn new (file_name: &str) -> file
    {
      file { file_name: String::from (file_name) }
    }
  }

  impl dir
  {
    pub fn new (dir_name: &str) -> dir
    {
      dir { dir_name: String::from (dir_name) }
    }
  }

  pub enum node_type
  {
    leaf (file),
    node (dir),
  }
}


#[cfg(test)]
mod tests {

  use super::node::node_type;
  use super::node::file;
  use super::node::dir;

    #[test]
    fn it_works() {
      node_type::leaf (file::new ("hatch"));
      node_type::node (dir::new ("hatch"));
    }
}
