use std::fs;
use std::io::prelude::*;use clap::{ App, SubCommand, ArgMatches };
use cli::commands::Command;
use deps::clone_project_deps;
use project::ProjectKind;
use deps::dependency::Dependency;
use locations::{ hatchfile_path, modules_path };
use hatch_error::{ HatchResult };
use task;
use cli::commands::{ VERSION, TYPE };
use cli::commands::parse_dependencies;
use project::CompilerOptions;
use project::Project;
use serde_yaml;
use std::fs::File;

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
    value_t!(args, VERSION, String).unwrap_or("0.0.1".to_owned())
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
  }

  fn generate_project(&self, args: &ArgMatches<'new>) -> HatchResult<()> {
    let name = self.project_name(args).unwrap_or("".to_string());
    let version = self.project_version(args);
    let kind = self.project_kind(args);

    let dir_path = self.project_path(args).join(&name);
    let hatch_file = hatchfile_path(&dir_path);

    let includes = parse_dependencies(args);

    // create the hatch project file structure
    fs::create_dir(&dir_path)?;
    fs::create_dir(modules_path(&dir_path))?;
    fs::create_dir(dir_path.join("src"))?;
    fs::create_dir(dir_path.join("target"))?;
    fs::create_dir_all(dir_path.join("test").join("src"))?;
    fs::create_dir(dir_path.join("test").join("target"))?;

    // clone dependencies
    if !includes.is_empty() {
      println!("Installing project dependencies...");
      clone_project_deps(modules_path(&dir_path).as_path(), &includes)?;
    }

    // create hatch file
    println!("Creating Hatch.yml file...");
    let compiler_options = CompilerOptions::default_from_kind(&kind);
    let project = Project::new(name, version, kind, compiler_options, includes);
//    let yaml_output = serde_yaml::to_string(&project)?;
//    let mut file = fs::File::create(hatch_file)?;
//    file.write_all(yaml_output.as_bytes())?;

    // Generate Assets
    println!("Generating assets...");
    task::generate_assets(&project)?;

    println!("Finished");

    Ok(())
  }
}

impl<'command> Command<'command> for New {
  fn cli_subcommand(&self) -> App<'command, 'command> {
    // SubCommand::from_yaml("")
    SubCommand::with_name(&self.name)
  }

  fn subcommand_name(&self) -> &'static str {
    self.name
  }

  fn execute(&self, args: &ArgMatches<'command>) -> HatchResult<()> {

    let project_result = New::generate_project(&self, args);
//    let execute_result = project_result.with_context(|e| {
//      format!("Failed to create project at: `{}` : {}", dir_path.display(), e)
//    });
    project_result
  }

//  fn generate_project(&self, args: &ArgMatches<'command>) -> HatchResult<()> {
//    let name = self.project_name(args).unwrap();
//    let version = self.project_version(args);
//    let kind = self.project_kind(args);
//
//    let dir_path = self.project_path(args).join(&name);
//    let hatch_file = hatchfile_path(&dir_path);
//
//    let dependencies = parse_dependencies(args);
//
//    // create the hatch project file structure
//    fs::create_dir(&dir_path)?;
//    fs::create_dir(modules_path(&dir_path))?;
//    fs::create_dir(dir_path.join("src"))?;
//    fs::create_dir(dir_path.join("target"))?;
//    fs::create_dir_all(dir_path.join("test").join("src"))?;
//    fs::create_dir(dir_path.join("test").join("target"))?;
//
//    // clone dependencies
//    if !dependencies.is_empty() {
//      println!("Installing project dependencies...");
//      clone_project_deps(modules_path(&dir_path).as_path(), &dependencies)?;
//    }
//
//    // create hatch file
//    println!("Creating Hatch.yml file...");
//    let includes = self.construct_deps_string(&deps_from_cli);
//
//    let compiler_options = CompilerOptions::default_from_kind(&kind);
//
//    let project = Project::new(name, version, kind, compiler_options, includes);
//    let yaml_output = serde_yaml::to_string(&project).unwrap();
//    let mut file = fs::File::create(hatch_file)?;
//    file.write_all(yaml_output.as_bytes())?;
//
//    // Generate Assets
//    println!("Generating assets...");
//    task::generate_assets(&project)?;
//
//    println!("Finished");
//    // let projekt = Projekt::from_args(args)?;
//
//    // projekt.generateHatchfile()?;
//    // projekt.generateAssets()?;
//
//    let res = (|| -> HatchResult<()> {
//      Ok(())
//    })().with_context(|e| {
//      format!("Failed to create project at: `{}` : {}", dir_path.display(), e)
//    })?;
//
//    Ok(res)
//  }
}
