use project::{ Project };
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
  pub fn tupfile(project: &Project) -> TestAssets {
    let file_path = project.path().to_string() + "/" + project.name() + "/test/Tupfile";
    let file_contents =
".gitignore".to_string();

    TestAssets { file_path, file_contents }
  }
}
