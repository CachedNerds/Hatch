pub struct Tupfile;

impl Tupfile {
  pub fn new() -> Tupfile {
    Tupfile
  }
}

impl ToString for Tupfile {
  fn to_string(&self) -> String {
    String::from(".gitignore")
  }
}
