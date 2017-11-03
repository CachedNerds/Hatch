use dtl::tup::{ Assets };

pub struct TestAssets {
  file_path: String,
  file_contents: String,
}

impl Assets for TestAssets {
  fn path(&self) -> &str {
    &self.file_path.as_str()
  }

  fn contents(&self) -> &str {
    &self.file_contents.as_str()
  }
}

impl TestAssets {
  pub fn tupfile(path: &str, name: &str) -> TestAssets {
    let file_path = path.to_string() + "/" + name + "/test/Tupfile";
    let file_contents =
".gitignore".to_string();

    TestAssets { file_path, file_contents }
  }
}
