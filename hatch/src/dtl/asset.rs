pub struct AssetOptions {
  input_file: String,
  read: bool,
  write: bool,
  output_file: String,
}

impl AssetOptions {
  pub fn new() -> AssetOptions {
    AssetOptions {
      input_file: String::new(),
      read: false,
      write: false,
      output_file: String::new(),
    }
  }
}
