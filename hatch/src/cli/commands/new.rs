use std::fs;
use std::io::prelude::*;
use clap::{ ArgMatches };
use cli::commands::{ Command };
use deps::clone_project_deps;
use locations::{ hatchfile_path, modules_path };
use hatch_error::{ HatchResult };
use task;
use project::{ CompilerOptions, Project };
use serde_yaml;
use generators::tup::Tup;

pub struct New;

impl<'new> New {
  pub fn new() -> New {
    New
  }
}

impl<'command> Command<'command> for New {
  fn execute(&self, args: &ArgMatches<'command>) -> HatchResult<()> {
    let name = self.project_name(args).unwrap_or("".to_string());
    let version = self.project_version(args);
    let kind = self.project_kind(args);
    let dir_path = self.project_path(args).join(&name);
    let hatch_file = hatchfile_path(&dir_path);
    let includes = self.parse_dependencies(args);
    fs::create_dir(&dir_path)?;
    fs::create_dir(modules_path(&dir_path))?;
    fs::create_dir(dir_path.join("src"))?;
    fs::create_dir(dir_path.join("target"))?;
    fs::create_dir_all(dir_path.join("test").join("src"))?;
    fs::create_dir(dir_path.join("test").join("target"))?;
    if !includes.is_empty() {
      println!("Installing project dependencies...");
      clone_project_deps(modules_path(&dir_path).as_path(), &includes)?;
    }
    println!("Creating Hatch.yml file...");
    let compiler_options = CompilerOptions::default_from_kind(&kind);
    let project = Project::new(name, version, kind, compiler_options, includes);
    let yaml_output = serde_yaml::to_string(&project)?;
    let mut file = fs::File::create(hatch_file)?;
    file.write_all(yaml_output.as_bytes())?;
    println!("Generating assets...");
    let generator = Box::new(Tup{});
    self.generate_assets(generator, dir_path, &project)?;
    println!("Finished");
    Ok(())
  }
}
