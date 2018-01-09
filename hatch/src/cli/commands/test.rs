use std::process;
use cli::commands::Command;
use cli::commands::ops::ProjectOps;
use HatchResult;
use hatch_error::HatchError;
use project::Project;
use clap::{ App, SubCommand, Arg, ArgMatches };

struct ImplicitTester;
struct ExplicitTester;

impl ImplicitTester {
  fn execute(&self, path: String, _: Vec<String>) {

  }
}

impl ExplicitTester {
  fn execute(&self, path: String, project_names: Vec<String>) {
    for project_name in project_names {
      let testExecutablePath = path.to_owned() + "test/build/" + project_name.as_str() + ".test";

      let result = process::Command::new(testExecutablePath)
                                     .arg("--success")
                                     .output();

      match result {
        Ok(output) => {
          println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
        },
        Err(e) => {
          println!("{:?}", e);
        },
      }
    }
  }
}

pub struct Test {
  name: &'static str,
}

impl<'test> Test {
  pub fn new() -> Test {
    Test {
      name: "test",
    }
  }
}

impl<'command> Command<'command> for Test {
  fn cli_subcommand(&self) -> App<'command, 'command> {
    SubCommand::with_name(&self.name)
      .about("Tests a project.")
      .author("Danny Peck <danieljpeck93@gmail.com>")

      .arg(Arg::with_name("PROJECT_NAMES")
        .help("The projects to be tested.")
        .min_values(0)
        .value_delimiter(" ")
        .required(false))
  }
  
  fn subcommand_name(&self) -> &'static str {
    self.name
  }

  fn execute(&self, args: &ArgMatches<'command>) -> Vec<HatchResult<Project>> {
    if args.is_present("PROJECT_NAMES") {
      let tester = Box::new(ExplicitTester);
      tester.execute(self.project_path(args), self.project_names(args));
    } else {
      let tester = Box::new(ImplicitTester);
      tester.execute(self.project_path(args), self.project_names(args));
    }

    vec![]
  }
}