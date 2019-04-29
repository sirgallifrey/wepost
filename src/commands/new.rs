use crate::entities::{HttpMethods, RequestDefinition};
use clap::{App, Arg, ArgMatches, SubCommand};
use std::default::Default;
use std::fs;
use std::path::PathBuf;

pub static NAME: &'static str = "new";
pub static ARGS_TEMPLATE: &'static str = "template";
pub static ARGS_OUTPUTDIR: &'static str = "output directory";
pub static ARGS_REQUESTNAME: &'static str = "REQUEST_NAME";
pub static ARGS_URL: &'static str = "URL";

arg_enum! {
    enum Templates {
        GET,
        POST
    }
}

impl Default for Templates {
    fn default() -> Templates {
        Templates::GET
    }
}

pub fn get_definition<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("new")
        .about("create a new file that describes a request")
        .arg(
            Arg::with_name(ARGS_TEMPLATE)
                .help("Template for the file")
                .short("t")
                .takes_value(true)
                .possible_values(&Templates::variants())
                .case_insensitive(true)
                .default_value("GET")
                .long("template"),
        )
        .arg(
            Arg::with_name(ARGS_OUTPUTDIR)
                .short("d")
                .long("directory")
                .help("Where to create the file")
                .required(false)
                .takes_value(true)
                .default_value("./")
        )
        .arg(
            Arg::with_name(ARGS_REQUESTNAME)
                .help("The name for this request, ex: get_all. It will be used for the filename")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::with_name(ARGS_URL)
                .help("The url to make the request to")
                .required(false)
                .index(2),
        )
}

pub fn execute_command<'a>(matches: &ArgMatches<'a>) {
    let template =
        value_t!(matches.value_of(ARGS_TEMPLATE), Templates).unwrap_or_else(|e| e.exit());
    let output = matches.value_of(ARGS_OUTPUTDIR).unwrap();
    let request_name = matches.value_of(ARGS_REQUESTNAME).unwrap();
    let url = matches
        .value_of(ARGS_URL)
        .unwrap_or("http://localhost/your/path/here");

    let mut path = PathBuf::from(output);
    path.push(&request_name);
    path.set_extension("wepost.json");

    let data = match template {
        Templates::GET => {
            RequestDefinition::new(url.to_owned(), HttpMethods::GET)
        }
        Templates::POST => {
            let mut req = RequestDefinition::new(url.to_owned(), HttpMethods::POST);
            req.headers.insert("Content-Type".to_owned(), "Application/Json".to_owned());
            req
        }
    };

    let serialized = serde_json::to_string_pretty(&data).unwrap();

    match fs::write(&path, &serialized) {
        //TODO: don't swallow this error, return back to main so we can exit with an error code.
        Err(_e) => {
            println!("Unable to create file, check for ilegal filename or permission problems.");
            println!(
                "Output dir received: {} and filename received: {}",
                &output, &request_name
            );
            println!("Tryed to create file at: {}", &path.display());
        }
        Ok(_) => (),
    }
}
