use hatch_error::{CleanupError, HatchResult};
use clap::{App, ArgMatches, SubCommand};
use cli::commands::Command;
use task;
use project::Project;
use assets::builder::Builder as AssetBuilder;
use assets::janitor;

pub struct Clean {
    name: &'static str,
}

impl<'update> Clean {
    pub fn new() -> Clean {
        Clean { name: "clean" }
    }

    fn print_errors(&self, results: &Vec<HatchResult<()>>) {
        for result in results {
            match result.as_ref() {
                Ok(_) => {}
                Err(ref e) => {
                    println!("{}", e);
                }
            }
        }
    }

    // determine if the collection of results contains an error
    fn contains_errors(&self, results: &Vec<HatchResult<()>>) -> bool {
        for result in results {
            match result.as_ref() {
                Ok(_) => {}
                Err(_) => {
                    return true;
                }
            }
        }

        return false;
    }

    pub fn clean(&self, project: &Project) -> HatchResult<()> {
        let asset_builder = AssetBuilder::from(&project);

        let remove_assets_results = janitor::remove_assets(asset_builder.assets());
        let remove_targets_results = janitor::remove_targets(project.path());

        self.print_errors(&remove_assets_results);
        self.print_errors(&remove_targets_results);

        let errors_occurred = self.contains_errors(&remove_assets_results)
            || self.contains_errors(&remove_targets_results);
        if errors_occurred {
            Err(CleanupError)?
        } else {
            Ok(())
        }
    }
}

impl<'command> Command<'command> for Clean {
    fn cli_subcommand(&self) -> App<'command, 'command> {
        SubCommand::with_name(&self.name)
            .about("Removes volatile project artifacts.")
            .version("0.1.0")
            .author("Danny Peck <danieljpeck93@gmail.com>")
    }

    fn subcommand_name(&self) -> &'static str {
        self.name
    }

    fn execute(&self, args: &ArgMatches<'command>) -> HatchResult<()> {
        let project = task::read_project(&self.project_path(args))?;
        self.clean(&project)?;

        Ok(())
    }
}
