#[macro_use]
extern crate clap;
extern crate serde;
extern crate serde_json;
extern crate glob;
use clap::{App};
mod commands;
mod entities;

fn main() {
    let matches = App::new("WePost automating your requests")
        .version("0.0.1")
        .author("Adilson Schmitt Junior")
        .subcommand(
            commands::new::get_definition()
        )
        .subcommand(
            commands::run::get_definition()
        )
        .get_matches();
    
    if let Some(new_matches) = matches.subcommand_matches(commands::new::NAME) {
        commands::new::execute_command(new_matches);
    }

    if let Some(run_matches) = matches.subcommand_matches(commands::run::NAME) {
        commands::run::execute_command(run_matches);
    }
}
