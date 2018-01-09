use HatchResult;
use std::fs;
use clap::{ App, SubCommand, Arg, ArgMatches };
use cli::commands::{ Command };
use project::{ Project, ProjectKind, LibraryKind };
use hatch_error::HatchError;

// Must use qualified names to avoid conflict.
use std::fmt::Write as FmtWrite;
use std::io::Write as IoWrite;

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
    if args.is_present("VERSION") {
      value_t!(args, "VERSION", String).unwrap()
    } else {
      "0.0.1".to_owned()
    }
  }

  fn project_kind(&self, args: &ArgMatches<'new>) -> ProjectKind {
    if args.is_present("bin") {
      ProjectKind::Binary
    } else if args.is_present("static") {
      ProjectKind::Library(LibraryKind::Static)
    } else {
      ProjectKind::Library(LibraryKind::Shared)
    }
  }

  fn get_includes(&self, args: &ArgMatches<'new>) -> Vec<String> {
    match args.values_of("include") {
      Some(values) => values.map(String::from).collect(),
      None => vec![],
    }
  }
}

impl<'command> Command<'command> for New {
  fn cli_subcommand(&self) -> App<'command, 'command> {
    SubCommand::with_name(&self.name)
      .about("Creates a new project. (default = shared library)")

      .arg(Arg::with_name("PROJECT_NAMES")
           .help("Name of project")
           .takes_value(true).max_values(1)
           .required(true))

      .arg(Arg::with_name("bin")
           .help("Generate a stand alone executable")
           .long("bin").short("b")
           .required(false)) 

      .arg(Arg::with_name("static")
           .help("Generate a static library")
           .long("static").short("s").conflicts_with("bin")
           .required(false))

      .arg(Arg::with_name("VERSION")
           .help("Set the project version")
           .long("version").short("v").takes_value(true)
           .required(false))

      .arg(Arg::with_name("INCLUDE")
           .help("List URLs to git repositories")
           .long("include")
           .short("i")
           .min_values(1)
           .takes_value(true))
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
        let includes_str =  String::from("  - ") + &self.get_includes(args).join("\n  - ");

        let mut yaml_output = String::new();

        let _ = write!(&mut yaml_output, "name: {}\nversion: {}\nbuild: {}\ndeps:\n{}",
                       &name, &version, &kind, &includes_str);

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
