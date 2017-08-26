#[macro_use]
extern crate clap;

mod cli;

fn main() {
  match cli::build_cli().get_matches().subcommand() {
    ("new", Some(args)) => {
      println!("Project Name: {}", value_t!(args, "PROJECT_NAME", String).unwrap());
      
      if args.is_present("bin") {
        println!("Generating binary project");
      } else {
        println!("Generating library project");
      }
    },
    _                   => println!("under construction"),
  }
}
