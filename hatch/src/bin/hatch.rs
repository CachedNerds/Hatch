extern crate hatch;

use hatch::cli::Cli;
use hatch::project::Project;

fn main() {
  let cli = Cli::new();
  let project = Project::new(cli.name(), cli.build_type(), cli.path());
}
