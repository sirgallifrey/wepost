#[macro_use]
extern crate clap;
use clap::{App};
mod commands;

fn main() {
    let matches = App::new("WePost automating your requests")
        .version("0.0.1")
        .author("Adilson Schmitt Junior")
        .subcommand(
            commands::new::get_definition()
        )
        .get_matches();
    
    if let Some(new_matches) = matches.subcommand_matches(commands::new::NAME) {
        commands::new::execute_command(new_matches);
    }
}
