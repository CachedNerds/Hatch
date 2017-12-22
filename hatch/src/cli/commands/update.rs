use clap::{ App, SubCommand, Arg, ArgMatches };
use cli::commands::Command;

use yaml;

use manifest::Manifest;

use yaml_rust::{ Yaml, YamlLoader };
use yaml_rust::yaml::Hash;

use project::{ Project, LibraryKind, ProjectKind };

use hatch_error::{
  HatchError,
  MissingNameError,
  MissingBuildError,
  MissingVersionError,
  EmptyConfigError
};

pub struct Update {
  name: &'static str
}

impl Update {
  pub fn new() -> Update {
    Update { name: "update" }
  }

  fn read_config(&self, yml_vec: Vec<Yaml>, path: String) -> Result<Manifest, HatchError> {
    if yml_vec.len() == 0 {
      return Err(HatchError::EmptyConfig(EmptyConfigError));
    }

    let name: String;
    let kind: ProjectKind;
    let version: String;

    if let Some(n) = yml_vec[0]["name"].as_str() {
      name = n.to_owned();
    } else {
      return Err(HatchError::MissingName(MissingNameError));
    }

    if let Some(b) = yml_vec[0]["build"].as_str() {
      kind = match b {
        "static-lib" => ProjectKind::Library(LibraryKind::Shared), 
        "shared-lib" => ProjectKind::Library(LibraryKind::Static),
        _ => ProjectKind::Binary
      }
    } else {
      return Err(HatchError::MissingBuild(MissingBuildError));
    }

    if let Some(v) = yml_vec[0]["version"].as_str() {
      version = v.to_owned();
    } else {
      return Err(HatchError::MissingVersion(MissingVersionError));
    }

    let project = Project::new(name, kind, path, version);

    Ok(Manifest::new(project))
  }
}

impl<'command> Command<'command> for Update {
  fn cli_subcommand(&self) -> App<'command, 'command> {
    SubCommand::with_name(&self.name)
      .about("Updates project dependencies.")
      .version("0.1.0")
      .author("Josh Gould <mrgould93@gmail.com>") 
  }

  fn subcommand_name(&self) -> &'static str {
    self.name
  }

  fn execute(&self, args: &ArgMatches<'command>) -> Result<Manifest, HatchError> {
    match yaml::from_file(self.project_path(args) + "Hatch.yml") {
      Ok(yml_vec) => {
        self.read_config(yml_vec, self.project_path(args))
      },
      Err(e) => Err(HatchError::from(e)),
    }
  }
}
