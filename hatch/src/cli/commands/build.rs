use HatchResult;

use std::fs;
use std::ffi::OsString;
use std::ffi::OsStr;
use std::path::PathBuf;

use clap::{ App, SubCommand, Arg, ArgMatches };
use cli::commands::Command;
use cli::commands::ops::ProjectOps;
use yaml;
use project::Project;
use hatch_error::HatchError;

struct ImplicitBuilder;
struct ExplicitBuilder;

impl ImplicitBuilder {
  fn get_project_names(&self, dir_paths: Vec<PathBuf>) -> Vec<String> {
    dir_paths.iter()
      .filter_map(|i| i.file_name())
      .map(OsStr::new)
      .filter_map(|i| i.to_str())
      .map(String::from)
      .collect()
  }

  fn extract_dirs(&self, iter: fs::ReadDir) -> Vec<PathBuf> {
    iter.filter_map(|i| i.ok())
      .into_iter()
      .map(|i| i.path())
      .filter(|i| i.is_dir())
      .collect()
  }

  fn read_path(&self, path: &str) -> HatchResult<fs::ReadDir> {
    match fs::read_dir(path) {
      Ok(iter) => Ok(iter),
      Err(e) => Err(HatchError::from(e)),
    }
  }

  fn parse_all(&self, path: &String) -> Vec<HatchResult<Project>> {
    match self.read_path(path) {
      Ok(files) => yaml::parse_many(path, self.get_project_names(self.extract_dirs(files))),
      Err(e) => vec![Err(e)],
    }
  }
}

impl ProjectOps for ImplicitBuilder {
  fn execute(&self, path: String, _: Vec<String>) -> Vec<HatchResult<Project>> {
    match yaml::parse_one(&path) {
      Ok(project) => vec![Ok(project)],
      Err(e) => self.parse_all(&path),
    }
  }
}

impl ProjectOps for ExplicitBuilder {
  fn execute(&self, path: String, project_names: Vec<String>) -> Vec<HatchResult<Project>> {
    yaml::parse_many(&path, project_names)
  }
}

pub struct Build {
  name: &'static str
}

impl<'build> Build {
  pub fn new() -> Build {
    Build {
      name: "build"
    }
  }
}

impl<'command> Command<'command> for Build {
  fn cli_subcommand(&self) -> App<'command, 'command> {
    SubCommand::with_name(&self.name)
      .about("Builds a project.")
      .author("Josh Gould <mrgould93@gmail.com>")

      .arg(Arg::with_name("PROJECT_NAMES")
           .help("The projects to be built.")
           .required(false)
           .min_values(0)
           .value_delimiter(" "))
  }

  fn subcommand_name(&self) -> &'static str {
    self.name
  }

  fn execute(&self, args: &ArgMatches<'command>) -> Vec<HatchResult<Project>> {
    let builder: Box<ProjectOps>;

    if args.is_present("PROJECT_NAMES") {
      builder = Box::new(ExplicitBuilder);
    } else {
      builder = Box::new(ImplicitBuilder);
    }

    builder.execute(self.project_path(args), self.project_names(args))
  }
}
