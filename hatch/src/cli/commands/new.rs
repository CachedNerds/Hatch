use std::fs;
use clap::{ App, SubCommand, Arg, ArgMatches };
use cli::commands::{ Command, parse_deps_from_cli };
use repo::{ clone_dep, modules_path };
use project::{ Project, ProjectKind, LibraryKind };
use hatch_error::{ HatchResult, ResultExt };

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

  fn construct_deps_string(&self, args: &ArgMatches<'new>) -> String {
    parse_deps_from_cli(args).into_iter().map(|(url, name)| {
      let mut includes = String::new();
      includes.push_str("  - ");
      includes.push_str(&name);
      includes.push_str("\n");
      includes.push_str("  ");
      includes.push_str(&url);
      includes.push_str("\n");
      includes
    }).collect()
  }

  fn hatch_yml_contents(&self, name: &str, version: &str, kind: &ProjectKind, includes: &str)-> String {
    let mut yaml_output = String::new();

    let _ = write!(&mut yaml_output, "name: {}\nversion: {}\nbuild: {}\ndeps:\n{}",
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

  fn execute(&self, args: &ArgMatches<'command>) -> HatchResult<Project> {
    let name = self.project_name(args).unwrap();

    let dir_path = self.project_path(args) + &name[..];
    let hatch_file = self.project_path(args) + &name[..] + "/Hatch.yml";

    let version = self.project_version(args);
    let kind = self.project_kind(args);
    
    let res = (|| -> HatchResult<_> {
      fs::create_dir(&dir_path)?;
      fs::create_dir(modules_path(&dir_path))?;
      
      if args.is_present(INCLUDE) {
        parse_deps_from_cli(args).iter().for_each(|repo| {
          clone_dep(repo, &modules_path(&dir_path));
        });
      }
      
      let includes = self.construct_deps_string(args);
      let yaml_output = self.hatch_yml_contents(&name, &version, &kind, &includes);

      let mut file = fs::File::create(hatch_file)?;
      file.write_all(yaml_output.as_bytes())?;
      
      Ok(Project::new(name, kind, version))
    })().with_context(|e| {
      format!("Failed to create project at: `{}` : {}", dir_path, e)
    })?;
    Ok(res)
  }
}
