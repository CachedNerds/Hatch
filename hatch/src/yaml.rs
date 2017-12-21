use std::fs;
use std::io::Read;

use yaml_rust::{ Yaml, YamlLoader };
use yaml_rust::scanner::{ ScanError };

use std::error;

use hatch_error::HatchError;
use self::HatchError::{ Io, Parsing };

pub fn from_file(file_name: String) -> Result<Vec<Yaml>, HatchError> {
  let parsed = fs::File::open(&file_name).and_then(|mut file| {
    let mut contents = String::new();
    file.read_to_string(&mut contents).map(|_| YamlLoader::load_from_str(&contents))
  });

  match parsed {
    Err(ioError) => Err(Io(ioError)),
    Ok(yaml_result) => match yaml_result {
      Err(scanner_error) => Err(Parsing(scanner_error)),
      Ok(vec_yaml) => Ok(vec_yaml)
    }
  }
}
