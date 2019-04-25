extern crate clap;
use clap::{Arg, App, SubCommand};

fn main() {
    let matches = App::new("WePost automating your requests")
        .version("0.0.1")
        .author("Adilson Schmitt Junior")
        .subcommand(SubCommand::with_name("new")
            .about("create a new file that describes a request")
            .arg(Arg::with_name("OUTPUT_FILE")
                .help("Where to create the file")
                .required(true)
                .last(true)
            )
            .arg(Arg::with_name("template")
                .help("Template for the file")
                .short("t")
                .long("template")
                .default_value("GET")
            )
            
        )
        .get_matches();
}
