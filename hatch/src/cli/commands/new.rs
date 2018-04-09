use std::fs;
use clap::{ App, SubCommand, Arg, ArgMatches };
use cli::commands::Command;
use deps::clone_project_deps;
use project::ProjectKind;
use deps::dependency::Dependency;
use locations::{ hatchfile_path, modules_path };
use hatch_error::{ HatchResult, ResultExt };
use task;
use cli::commands::{ INCLUDE, VERSION, TYPE, BIN, STATIC, SHARED, HEADER, PROJECT_NAME };
use cli::commands::parse_dependencies;
use project::CompilerOptions;
use project::Project;

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
    if args.is_present(TYPE) {
      let type_arg: String = value_t!(args, TYPE, String).unwrap();
      ProjectKind::from_str(type_arg.as_str())
    } else {
      ProjectKind::default()
    }
  }

  fn construct_deps_string(&self, project_deps: &Vec<Dependency>) -> String {
    project_deps.iter().map(Dependency::to_string).collect::<Vec<_>>().join("\n")
//    if project_deps.is_empty() {
//      String::new()
//    } else {
//      String::from("deps:\n") + project_deps.iter().map(|d| {
//        format!("  {}: {}\n", d.name(), d.url())
//      }).collect::<String>().as_str()
//    }
  }

//  fn hatch_yml_contents(&self,
//                        name: &str,
//                        version: &str,
//                        config: &Config,
//                        includes: &str) -> String
//  {
//    let mut yaml_output = String::new();
//
//    let _ = write!(&mut yaml_output,
//"name: {}
//version: {}
//build:
//  kind: {}
//  compiler: {}
//  compiler_flags: {}
//  linker_flags: {}
//  arch: {}
//  target: {}
//{}",
//                   &name,
//                   &version,
//                   config.kind(),
//                   config.compiler(),
//                   config.compiler_flags().join(" "),
//                   config.linker_flags().join(" "),
//                   config.arch(),
//                   config.target(),
//                   &includes);
//    yaml_output
//  }
//}
}

impl<'command> Command<'command> for New {
  fn cli_subcommand(&self) -> App<'command, 'command> {
    SubCommand::with_name(&self.name)
      .about("Creates a new project. (default = static library)")

      .arg(Arg::with_name(PROJECT_NAME)
           .help("Name of project")
           .takes_value(true)
           .required(true))

      .arg(Arg::with_name(TYPE)
           .help("Determines the type of the project")
           .long("type").short("t")
           .takes_value(true)
           .possible_values(&[BIN, STATIC, SHARED, HEADER])
           .required(true))

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

    let deps_from_cli = parse_dependencies(args);
    
    let res = (|| -> HatchResult<()> {
      println!("Creating directory structure...");

      // create the hatch project file structure
      fs::create_dir(&dir_path)?;
      fs::create_dir(modules_path(&dir_path))?;
      fs::create_dir(dir_path.join("src"))?;
      fs::create_dir(dir_path.join("target"))?;
      fs::create_dir_all(dir_path.join("test").join("src"))?;
      fs::create_dir(dir_path.join("test").join("target"))?;

      if !project_deps.is_empty() {
        println!("Installing project dependencies...");
        clone_project_deps(modules_path(&dir_path).as_path(), &project_deps)?;
      }

      let includes = self.construct_deps_string(&deps_from_cli);
      // let config = self.construct_config(kind);

      let compiler_options = CompilerOptions::default();

      // let config = Project::new(name, version, Some(compiler_options), deps_from_cli);


//      let yaml_output = self.hatch_yml_contents(&name,
//                                                &version,
//                                                config.as_ref(),
//                                                &includes);

      println!("Creating Hatch.yml file...");
      let mut file = fs::File::create(hatch_file)?;
      file.write_all(yaml_output.as_bytes())?;

      let project = Project::new(name,
                                 version,
                                 kind,
                                 Some(compiler_options),
                                 project_deps);

      println!("Generating assets...");
      task::generate_assets(&project)?;

      println!("Finished");

      Ok(())
    })().with_context(|e| {
      format!("Failed to create project at: `{}` : {}", dir_path.display(), e)
    })?;

    Ok(res)
  }
}
