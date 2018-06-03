

use clap::ArgMatches;
use constants::{ARGS, INCLUDE, PROJECT_NAME, PROJECT_PATH, TYPE, VERSION};
use deps::dependency::Dependency;
use failure::ResultExt;
use generators::platform_kind::PlatformKind;
use generators::Generator;
    fn execute(&self, args: &ArgMatches<'command>) -> HatchResult<()>;
    fn project_name(&self, args: &ArgMatches<'command>) -> Option<String> {
        value_t!(args, PROJECT_NAME, String).ok()
