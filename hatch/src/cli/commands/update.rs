use clap::{ App, SubCommand, Arg, ArgMatches };
use cli::commands::Command;

use std::io::{ Error, Read };
use std::fs;

use yaml_rust::{ Yaml, YamlLoader };
use yaml_rust::yaml::Hash;
use yaml_rust::scanner::{ ScanError };

use project::{ Project, LibraryKind, ProjectKind };
use config::contexts;

pub struct Update {
  name: &'static str
}

impl Update {
  pub fn new() -> Update {
    Update { name: "update" }
  }
}

impl<'command> Command<'command> for Update {
  fn cli_subcommand(&self) -> App<'command, 'command> {
    SubCommand::with_name(&self.name)
      .about("Updates project dependencies.")
      .version("0.1.0")
      .author("Mackenzie Clark <mackenzie.a.z.c@gmail.com>")
      
      .arg(Arg::with_name("PROJECT_NAME")
           .help("Name of project")
           .required(false)
           .takes_value(true))
  }

  fn subcommand_name(&self) -> &'static str {
    self.name
  }

  fn execute(&self, args: &ArgMatches<'command>) {
    match self.parse_hatch_yml(self.toolbox_path(args)) {
      Some(entry) => {
        let build = entry["build"].as_str().unwrap();
        
        let kind = match build {
          "static-lib" => ProjectKind::Library(LibraryKind::Shared), 
          "shared-lib" => ProjectKind::Library(LibraryKind::Static),
          _ => ProjectKind::Binary
        };

        let name = entry["name"].as_str().unwrap().to_owned();
        let version = entry["version"].as_str().unwrap().to_owned();

        let p = Project::new(name, kind, self.toolbox_path(args), version);
        contexts::update::Update::new(p);

      },
      None => println!("No Hatch.yml in {}", self.toolbox_path(args).as_str())
    }
  }
}

impl Update {
  fn parse_hatch_yml(&self, path: String) -> Option<Yaml> {
    let parsed = fs::File::open(path + "Hatch.yml").and_then(|mut file| {
      let mut contents = String::new();
      file.read_to_string(&mut contents).map(|_| contents)
    }).map(|mut s| YamlLoader::load_from_str(&mut s));

    match parsed {
      Err(_) => None,
      Ok(p) => p.unwrap().into_iter().nth(0)
    }
  }
}
