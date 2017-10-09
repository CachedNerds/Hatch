extern crate clap;

use std::path;
use super::errors;

#[derive(Debug)]
pub struct Metadata {
  pub version: (u16, u16, u16),
  pub path: path::PathBuf,
}

impl Metadata {
  pub fn new(args: &clap::ArgMatches) -> Result<Metadata, errors::Error> {
    Ok(Metadata {
      version: get_version(&args)?,
      path: get_path(&args)?,
    })
  }
}

fn get_path(args: &clap::ArgMatches) -> Result<path::PathBuf, errors::Error> {
  let mut path = path::PathBuf::new();

  if args.is_present("TOOLBOX_PATH") {
    path.push(value_t!(args, "TOOLBOX_PATH", String)?);
  } else {
    path.push("./");
  }

  path.push("C++/libs");

  Ok(path)
}

fn get_version(args: &clap::ArgMatches) -> Result<(u16, u16, u16), errors::Error> {
  match values_t!(args, "PROJECT_VERSION", u16) {
    Ok(v) => {
      if v.iter().count() == 3 { Ok((v[0], v[1], v[2])) }
      else {
        Err(errors::Error::from("Invalid version"))
      }
    },
    Err(_) => Ok((0, 0, 1))
  }
}
