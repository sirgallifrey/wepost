use clap::{App, Arg, ArgMatches, SubCommand};
use crate::entities::{HttpMethods, RequestDefinition};
use glob::glob;

pub static NAME: &'static str = "run";
pub static ARGS_GLOB: &'static str = "File glob patten";

pub fn get_definition<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name(NAME)
        .about("Executes http requests for matched files")
        .arg(
            Arg::with_name(ARGS_GLOB)
                .help("The files describing the requests to make. You can specify one file, directory or a glob pattern like my_dir/*.wepost.json")
                .required(true)
                .index(1),
        )
}

pub fn execute_command<'a>(matches: &ArgMatches<'a>) {
    let glob_patten = matches.value_of(ARGS_GLOB).unwrap();

    for entry in glob(glob_patten).expect("Failed to read glob pattern") {
        match entry {
            Ok(path) => println!("{:?}", path.display()),
            Err(e) => println!("{:?}", e),
        }
    }
}


