use hatch_error::HatchResult;
use clap::{App, Arg, ArgMatches, SubCommand};
use cli::commands::Command;
use task;

use cli::commands::PROJECT_NAMES;

pub struct Update {
    name: &'static str,
}

impl<'update> Update {
    pub fn new() -> Update {
        Update { name: "update" }
    }
}

impl<'command> Command<'command> for Update {
    fn cli_subcommand(&self) -> App<'command, 'command> {
        SubCommand::with_name(&self.name)
            .about("Updates project dependencies.")
            .version("0.1.0")
            .author("Josh Gould <mrgould93@gmail.com>")
            .arg(
                Arg::with_name(PROJECT_NAMES)
                    .help("The projects to be updated.")
                    .min_values(0)
                    .value_delimiter(" ")
                    .required(false),
            )
    }

    fn subcommand_name(&self) -> &'static str {
        self.name
    }

    fn execute(&self, args: &ArgMatches<'command>) -> HatchResult<()> {
        let project = task::read_project(&self.project_path(args))?;

        task::generate_assets(&project)?;

        Ok(())
    }
}
