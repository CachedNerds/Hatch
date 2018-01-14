use HatchResult;
use clap::{ App, SubCommand, Arg, ArgMatches };
use cli::commands::Command;
use cli::commands::ops::ProjectOps;
use yaml;
use project::Project;
use assets::{ Asset, TupKind, print_file_contents };
use assets::builder::Builder as AssetBuilder;

use cli::commands::PROJECT_NAMES;

struct ImplicitBuilder;
struct ExplicitBuilder;

impl ProjectOps for ImplicitBuilder {
  fn execute(&self, path: String, _: Vec<String>) -> Vec<HatchResult<Project>> {
    match yaml::parse_one(&path) {
      Ok(project) => vec![Ok(project)],
      Err(_) => yaml::parse_all(&path),
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

      .arg(Arg::with_name(PROJECT_NAMES)
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

    if args.is_present(PROJECT_NAMES) {
      builder = Box::new(ExplicitBuilder);
    } else {
      builder = Box::new(ImplicitBuilder);
    }

    let result = builder.execute(self.project_path(args), self.project_names(args));

    match result[0] {
      Ok(ref project) => {
        let mut asset_builder = AssetBuilder::from(project);
        for asset in asset_builder.assets() {
          println!("{}\n", asset.contents());
        }
      },
      _ => {},
    }

    vec![]
  }
}
