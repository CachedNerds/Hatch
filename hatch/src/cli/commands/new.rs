use std::fs;
use clap::{ App, SubCommand, Arg, ArgMatches };
use cli::commands::{ Command, parse_deps_from_cli };
use repo::{ clone_dep, modules_path, hatchfile_path, clone_project_deps };
use project::{ Project, ProjectKind, LibraryKind, Dependency };
use hatch_error::{ HatchResult, ResultExt };
use std::path::PathBuf;

// Must use qualified names to avoid conflict.
use std::fmt::Write as FmtWrite;
use std::io::Write as IoWrite;

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

  fn construct_deps_string(&self, deps: &Vec<(String, String)>) -> String {
    if deps.len() > 0 {
      String::from("deps:\n") + deps.iter().map(|&(ref url, ref name)| {
        format!("  {}: {}\n", &name[..], &url[..])
      }).collect::<String>().as_str()
    } else {
      String::new()
    }
  }

  fn hatch_yml_contents(&self,
                        name: &str,
                        version: &str,
                        kind: &ProjectKind,
                        includes: &str)-> String
  {
    let mut yaml_output = String::new();

    let _ = write!(&mut yaml_output, "name: {}\nversion: {}\nbuild: {}\n{}",
                   &name, &version, &kind, &includes);
    yaml_output
  }
}

impl<'command> Command<'command> for New {
  fn cli_subcommand(&self) -> App<'command, 'command> {
    SubCommand::with_name(&self.name)
      .about("Creates a new project. (default = shared library)")

      .arg(Arg::with_name(PROJECT_NAMES)
           .help("Name of project")
           .takes_value(true)
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

  fn execute(&self, args: &ArgMatches<'command>) -> HatchResult<Project> {
    let name = self.project_name(args).unwrap();
    let version = self.project_version(args);
    let kind = self.project_kind(args);

    let dir_path = PathBuf::from(self.project_path(args) + &name[..]);
    let hatch_file = PathBuf::from(hatchfile_path(&dir_path));

    let deps_from_cli = parse_deps_from_cli(args);
    
    let res = (|| -> HatchResult<_> {
      // create directory for new project
      fs::create_dir(&dir_path)?;
      // create the hatch_modules directory inside the project directory
      fs::create_dir(modules_path(&dir_path))?;

      let deps = deps_from_cli.iter().map(|repo| {
        Dependency::new(repo.1.clone(), repo.0.clone())
      }).collect::<Vec<_>>();

      if deps.len() > 0 {
        clone_project_deps(modules_path(&dir_path).as_path(), &deps)?;
      }

      let includes = self.construct_deps_string(&deps_from_cli);
      let yaml_output = self.hatch_yml_contents(&name, &version, &kind, &includes);

      let mut file = fs::File::create(hatch_file)?;
      file.write_all(yaml_output.as_bytes())?;

      Ok(Project::new(name, kind, version, deps))
    })().with_context(|e| {
      format!("Failed to create project at: `{}` : {}", dir_path.display(), e)
    })?;
    Ok(res)
  }
}
