#[derive(Debug)]
pub struct Version {
  version: (u16, u16, u16),
}

impl Version {
  fn from_str(string: &str) -> Version {
    let v = string.split('.').collect::<Vec<str>>();
    if v.iter().count() == 3 {
      Version { v[0], v[1], v[2] }
    } else {
      Version { 0, 0, 1 }
    }
  }
}
//
//fn get_version(args: &clap::ArgMatches) -> Result<(u16, u16, u16), errors::Error> {
//  match values_t!(args, "PROJECT_VERSION", u16) {
//    Ok(v) => {
//      if v.iter().count() == 3 { Ok((v[0], v[1], v[2])) }
//      else {
//        Err(errors::Error::from("Invalid version"))
//      }
//    },
//    Err(_) => Ok((0, 0, 1))
//  }
//}
//
impl Display for Version {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{} {} {}", self.version.0, self.version.1, self.version.2)
  }
}
