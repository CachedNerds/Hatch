use HatchResult;
use std::fs;
use std::collections::HashMap;
use clap::{ App, SubCommand, Arg, ArgMatches };
use cli::commands::{ Command };
use cli::commands::ops::RepoManager;
use project::{ Project, ProjectKind, LibraryKind };
use hatch_error::HatchError;

// Must use qualified names to avoid conflict.
use std::fmt::Write as FmtWrite;
use std::io::Write as IoWrite;

use git2;

use cli::commands::{ INCLUDE, VERSION, STATIC, BIN, PROJECT_NAMES };

pub struct New {
  name: &'static str,
}

impl<'new> New {
  pub fn new() -> New {
    New {
      name: "new",
    }
  }
  
  fn project_version(&self, args: &ArgMatches<'new>) -> String {
    if args.is_present(VERSION) {
      value_t!(args, VERSION, String).unwrap()
    } else {
      "0.0.1".to_owned()
    }
  }

  fn project_kind(&self, args: &ArgMatches<'new>) -> ProjectKind {
    if args.is_present(BIN) {
      ProjectKind::Binary
    } else if args.is_present(STATIC) {
      ProjectKind::Library(LibraryKind::Static)
    } else {
      ProjectKind::Library(LibraryKind::Shared)
    }
  }

  fn create_modules_directory(&self, path: &str) {
    let _ = fs::create_dir(self.modules_path(path));
  }
}

impl<'new> RepoManager<'new> for New {
  fn fetch_dependencies(&self, repo_spec: &Vec<(String, String)>, path: &str) {
    repo_spec.iter().for_each(|repo| {
      git2::Repository::clone(&repo.0, &repo.1);
    });
  }

  fn get_includes(&self, args: &ArgMatches<'new>) -> Vec<(String, String)> {
    let mut parsed_includes = Vec::new();
    match args.values_of(INCLUDE) {
      Some(values) => {
        let mut vals = values.map(String::from).collect::<Vec<String>>().into_iter();
        while vals.len() != 0 {
          let mut left = vals.next();
          let mut right = vals.next();
          parsed_includes.push((left.unwrap(), right.unwrap()));
        }
        parsed_includes
      },
      None => vec![],
    }
  }
}

impl<'command> Command<'command> for New {
  fn cli_subcommand(&self) -> App<'command, 'command> {
    SubCommand::with_name(&self.name)
      .about("Creates a new project. (default = shared library)")

      .arg(Arg::with_name(PROJECT_NAMES)
           .help("Name of project")
           .takes_value(true).max_values(1)
           .required(true))

      .arg(Arg::with_name(BIN)
           .help("Generate a stand alone executable")
           .long("bin").short("b")
           .required(false)) 

      .arg(Arg::with_name(STATIC)
           .help("Generate a static library")
           .long("static").short("s").conflicts_with("bin")
           .required(false))

      .arg(Arg::with_name(VERSION)
           .help("Set the project version")
           .long("version").short("v").takes_value(true)
           .required(false))

      .arg(Arg::with_name(INCLUDE)
           .help("List URLs to git repositories")
           .long("include").short("i").multiple(true).number_of_values(2).takes_value(true)
           .required(false))
  }

  fn subcommand_name(&self) -> &'static str {
    self.name
  }

  fn execute(&self, args: &ArgMatches<'command>) -> Vec<HatchResult<Project>> {
    let name = self.project_names(args).into_iter().nth(0).unwrap();

    let dir_path = self.project_path(args) + &name[..];
    let hatch_file = self.project_path(args) + &name[..] + "/Hatch.yml";

    match fs::create_dir(&dir_path) {
      Err(e) => vec![Err(HatchError::from(e))],
      Ok(_) => {
        let version = self.project_version(args);
        let kind = self.project_kind(args);

        self.create_modules_directory(&dir_path);

        if args.is_present(INCLUDE) {
          self.fetch_dependencies(&self.get_includes(args).as_mut(), &self.modules_path(&dir_path));
        }

        let mut includes = String::new();

        self.get_includes(args).iter().for_each(|&(ref a, ref b)| {
          includes.push_str("  - ");
          includes.push_str(&a);
          includes.push_str("\n");
        });

        let mut yaml_output = String::new();

        let _ = write!(&mut yaml_output, "name: {}\nversion: {}\nbuild: {}\ndeps:\n{}",
                       &name, &version, &kind, &includes);

        match fs::File::create(hatch_file) {
          Err(e) => vec![Err(HatchError::from(e))],
          Ok(mut file) => {
            match file.write_all(yaml_output.as_bytes()) {
              Err(e) => vec![Err(HatchError::from(e))],
              Ok(_) => vec![Ok(Project::new(name, kind, version))]
            }
          }
        }
      }
    }
  }
}
