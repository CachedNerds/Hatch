pub struct AssetOptions {
  input_file: String,
  read: bool,
  write: bool,
  output_file: Option<String>,
}

impl AssetOptions {
  pub fn new() -> AssetOptions {
    AssetOptions {
      input_file: String::new(),
      read: false,
      write: false,
      output_file: None,
    }
  }

  pub fn input_file(&mut self, file_name: &str) {
    self.input_file.push_str(file_name);
  }

  pub fn read(&mut self, read: bool) {
    self.read = read;
  }

  pub fn write(&mut self, write: bool) {
    self.write = write;
  }

  pub fn output_file(&mut self, file_name: Option<&str>) {
    if let Some(s) = file_name {
      self.output_file = Some(String::from(s));
    }
  }
}
