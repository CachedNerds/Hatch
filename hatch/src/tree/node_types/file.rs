pub struct File {
  contents: String,
  file_name: String,
}

impl File {
  pub fn new(file_name: &str) -> File {
    File {
      contents: "".to_string(),
      file_name: file_name.to_string()
    }
  }

  pub fn update_content(&mut self, data: &str) {
    self.contents += data;
  }
}
