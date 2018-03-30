use cli::commands::build::Build;
use cli::commands::Command;

#[test]
fn create_build_command() {
    let command = Build::new();
    let actual_name = command.subcommand_name().to_owned();
    let expected_name = String::from("build");
    assert_eq!(actual_name, expected_name);
}
