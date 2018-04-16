#[cfg(test)]
pub mod tests;

use std::path::Path;
use hatch_error::{ HatchResult };
use project::Project;
use serde_yaml;
use std::fs::File;
use std::io::Read;
//use assets::generator;
//use assets::builder::Builder;
use generators::Generator;
//use hatch_error::SerdeYamlError;

pub fn read_project(path: &Path) -> HatchResult<Project> {
  let mut data = String::new();
  File::open(path)?.read_to_string(&mut data)?;

  let res = serde_yaml::from_str::<Project>(&data);

  // This needs to be fixed, see the commented code below
  Ok(res.unwrap())

  // TODO: Why this no work? :frowning:
//  let realres: HatchResult<Project> = match res {
//    Ok(project) => Ok(project),
//    Err(e) => Err(SerdeYamlError)
//  };
//  realres
}

pub fn generate_assets(generator: Box<Generator>, project: &Project) -> HatchResult<()> {

  // get all the assets
  let assets = generator.asses();

  //let assets = Builder::from(project).assets();

  // generate those assets - this was a good idea!
  // the generator is completely independent of the asset generators
  // generator::generate_all(&assets);

//  generator::generate_all(Builder::from(project).assets()).with_context(|e| {
//    format!("asset generation failed : `{}`", e)
//  })?;



  Ok(())
}
