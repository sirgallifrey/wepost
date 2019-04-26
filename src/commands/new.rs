use clap::{App, Arg, SubCommand, ArgMatches};
use std::fs;
use std::default::Default;

pub static NAME: &'static str = "new";
pub static ARGS_TEMPLATE: &'static str = "template";
pub static ARGS_OUTPUTFILE: &'static str = "OUTPUT_FILE";

arg_enum!{
    enum Templates {
        GET,
        POST,
        BLANK
    }
}

impl Default for Templates {
    fn default() -> Templates { Templates::GET } 
}


pub fn get_definition<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("new")
        .about("create a new file that describes a request")
        .arg(
            Arg::with_name(ARGS_TEMPLATE)
                .help("Template for the file")
                .short("t")
                .long("template")
        )
        .arg(
            Arg::with_name(ARGS_OUTPUTFILE)
                .help("Where to create the file")
                .required(true),
        )
}

pub fn execute_command<'a>(matches: &ArgMatches<'a>) {
    let template = value_t!(matches.value_of(ARGS_TEMPLATE), Templates).unwrap_or_else(|e| e.exit());
    let output = matches.value_of(ARGS_OUTPUTFILE).unwrap();

    

    let data = match template {
        Templates::GET => "{\"path\":\"http://localhost/your/path/here\", \"method\":\"GET\" }",
        Templates::POST => "{\"path\":\"http://localhost/your/path/here\", \"method\":\"POST\", }",
        _ => ""
    };

    fs::write(output, data).expect(&format!("Unable to create file {}", output));
}
