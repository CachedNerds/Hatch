use std::fs;
use clap::{ App, SubCommand, Arg, ArgMatches };
use cli::commands::{ Command, parse_deps_from_cli };
use deps::clone_project_deps;
use project::{ Project, ProjectKind, LibraryKind };
use project::build::{ Target, Config };
use platform::arch::Arch;
use deps::dependency::Dependency;
use locations::{ hatchfile_path, modules_path };
use hatch_error::{ HatchResult, ResultExt };
use task;

// Must use qualified names to avoid conflict.
use std::fmt::Write as FmtWrite;
use std::io::Write as IoWrite;

use cli::commands::{ INCLUDE, VERSION, STATIC, BIN, PROJECT_NAME };

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

  fn construct_deps_string(&self, project_deps: &Vec<Dependency>) -> String {
    if project_deps.is_empty() {
      String::new()
    } else {
      String::from("deps:\n") + project_deps.iter().map(|d| {
        format!("  {}: {}\n", d.name(), d.url())
      }).collect::<String>().as_str()
    }
  }

  fn hatch_yml_contents(&self,
                        name: &str,
                        version: &str,
                        config: &Config,
                        includes: &str) -> String
  {
    let mut yaml_output = String::new();

    let _ = write!(&mut yaml_output,
"name: {}
version: {}
build:
  kind: {}
  compiler: {}
  compiler_flags: {}
  linker_flags: {}
  arch: {}
  target: {}
{}",
                   &name,
                   &version,
                   config.kind(),
                   config.compiler(),
                   config.compiler_flags().join(" "),
                   config.linker_flags().join(" "),
                   config.arch(),
                   config.target(),
                   &includes);
    yaml_output
  }
}

impl<'command> Command<'command> for New {
  fn cli_subcommand(&self) -> App<'command, 'command> {
    SubCommand::with_name(&self.name)
      .about("Creates a new project. (default = shared library)")

      .arg(Arg::with_name(PROJECT_NAME)
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
           .long("include").short("i").multiple(true).number_of_values(1).takes_value(true)
           .required(false))
  }

  fn subcommand_name(&self) -> &'static str {
    self.name
  }

  fn execute(&self, args: &ArgMatches<'command>) -> HatchResult<()> {
    let name = self.project_name(args).unwrap();
    let version = self.project_version(args);
    let kind = self.project_kind(args);

    let dir_path = self.project_path(args).join(&name);
    let hatch_file = hatchfile_path(&dir_path);

    let deps_from_cli = parse_deps_from_cli(args);
    
    let res = (|| -> HatchResult<()> {
      // create directory for new project
      fs::create_dir(&dir_path)?;
      // create the hatch_modules directory inside the project directory
      fs::create_dir(modules_path(&dir_path))?;

      fs::create_dir(dir_path.join("src"))?;
      fs::create_dir(dir_path.join("target"))?;
      fs::create_dir_all(dir_path.join("test").join("src"))?;
      fs::create_dir(dir_path.join("test").join("target"))?;

      let project_deps = deps_from_cli.into_iter().map(|repo| {
        Dependency::new(repo)
      }).collect::<Vec<_>>();

      if !project_deps.is_empty() {
        clone_project_deps(modules_path(&dir_path).as_path(), &project_deps)?;
      }

      let includes = self.construct_deps_string(&project_deps);

      let compiler: String = String::from("g++");
      let compiler_flags: Vec<String> = vec![String::from("-c")];
      let linker_flags: Vec<String> = vec![String::from("-v")];
      let mut arch: Arch = Arch::X64;
      if let Some(architecture) = Arch::architecture() {
        arch = architecture;
      }
      let target: Target = Target::Debug;

      let config = Config::new(kind, compiler, compiler_flags, linker_flags, arch, target);

      let yaml_output = self.hatch_yml_contents(&name,
                                                &version,
                                                config.as_ref(),
                                                &includes);

      let mut file = fs::File::create(hatch_file)?;
      file.write_all(yaml_output.as_bytes())?;

      let project = Project::new(name,
                                 version,
                                 config,
                                 project_deps,
                                 dir_path.to_owned());
      task::generate_assets(&project)?;

      Ok(())
    })().with_context(|e| {
      format!("Failed to create project at: `{}` : {}", dir_path.display(), e)
    })?;
    Ok(res)
  }
}
