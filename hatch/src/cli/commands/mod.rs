pub mod build;
pub mod new;
pub mod run;
pub mod test;
pub mod update;

use clap::ArgMatches;
use constants::{ARGS, INCLUDE, PROJECT_NAME, PROJECT_PATH, TYPE, VERSION, HATCH_FILE_NAME};
use deps::dependency::Dependency;
use failure::ResultExt;
use generators::platform_kind::PlatformKind;
use generators::Generator;
use hatch_error::HatchResult;
use hatch_error::InvalidPathError;
use platform::os;
use project::Project;
use project::ProjectKind;
use serde_yaml;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::path::PathBuf;
use std::process;

pub trait Command<'command> {
    fn execute(&self, args: &ArgMatches<'command>) -> HatchResult<()>;

    fn read_project_context(&self, args: &ArgMatches<'command>) -> HatchResult<(PathBuf, Project)> {
        let project_path = if args.is_present(PROJECT_PATH) {
            PathBuf::from(value_t!(args, PROJECT_PATH, String).unwrap().as_str())
        } else {
            PathBuf::from("./")
        };

        let project = {
            let project_path_ref = project_path.join(HATCH_FILE_NAME);
            let mut data = String::new();
            File::open(&project_path_ref)?.read_to_string(&mut data)?;
            serde_yaml::from_str::<Project>(&data)?
        };

        Ok((project_path, project))
    }

    fn project_name(&self, args: &ArgMatches<'command>) -> Option<String> {
        value_t!(args, PROJECT_NAME, String).ok()
    }

    fn project_path(&self, args: &ArgMatches<'command>) -> PathBuf {
        if args.is_present(PROJECT_PATH) {
            PathBuf::from(value_t!(args, PROJECT_PATH, String).unwrap().as_str())
        } else {
            PathBuf::from("./")
        }
    }

    fn project_version(&self, args: &ArgMatches<'command>) -> String {
        if args.is_present(VERSION) {
            value_t!(args, VERSION, String).unwrap()
        } else {
            "0.0.1".to_owned()
        }
    }

    fn project_kind(&self, args: &ArgMatches<'command>) -> ProjectKind {
        if args.is_present(TYPE) {
            let type_arg: String = value_t!(args, TYPE, String).unwrap();
            ProjectKind::from_str(type_arg.as_str())
        } else {
            ProjectKind::default()
        }
    }

    fn parse_arguments_from_cli(&self, cli_args: &ArgMatches<'command>) -> Vec<String> {
        if let Some(arguments) = cli_args.values_of(ARGS) {
            arguments.map(String::from).collect()
        } else {
            Vec::new()
        }
    }

    fn parse_dependencies<'func>(&self, args: &ArgMatches<'func>) -> Vec<Dependency> {
        if let Some(values) = args.values_of(INCLUDE) {
            values
                .map(String::from)
                .map(Dependency::new)
                .collect::<Vec<Dependency>>()
        } else {
            Vec::new()
        }
    }

    fn build(&self, project_path: &Path) -> HatchResult<()> {
        if let Some(path) = project_path.to_str() {
            let command = format!("cd {} && tup", path);
            let mut shell: String;
            let mut args: Vec<String>;
            match os::platform_type() {
                PlatformKind::Windows => {
                    shell = String::from("cmd");
                    args = vec![String::from("/C"), command];
                }
                _ => {
                    shell = String::from("sh");
                    args = vec![String::from("-c"), command];
                }
            }

            let mut child = process::Command::new(shell).args(args).spawn()?;
            child.wait()?;

            Ok(())
        } else {
            Err(InvalidPathError)?
        }
    }

    fn generate_assets(
        &self,
        generator: Box<Generator>,
        project_path: PathBuf,
        project: &Project,
    ) -> HatchResult<()> {
        generator
            .generate_assets(project_path, project)
            .with_context(|e| format!("asset generation failed : `{}`", e))?;
        Ok(())
    }
}
