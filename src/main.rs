#[macro_use]
extern crate clap;
extern crate aliases;

use aliases::App;

fn main() {
    let yaml = load_yaml!("../config/cli.yml"); // TODO this won't be included in the binary and has to be loaded another way
    let matches = clap::App::from_yaml(yaml).get_matches();

    match matches.subcommand_name() {
        Some("init") => {
            if let Some(matches) = matches.subcommand_matches("init") {
                App::new().execute_init(matches.is_present("global"));
            }
        },
        Some("list") => {
            if let Some(matches) = matches.subcommand_matches("list") {
                // TODO pass through the params here
                App::new().execute_list(matches.value_of("directory"), matches.value_of("name"));
            }
        },
        Some("rehash") => {
            if let Some(_) = matches.subcommand_matches("rehash") {
                App::new().execute_rehash();
            }
        },
        Some("exec") => {
            if let Some(matches) = matches.subcommand_matches("exec") {
                let directory = matches.value_of("directory").unwrap().to_string();
                let command_name = matches.value_of("name").unwrap().to_string();
                App::new().execute_exec(directory, command_name);
            }
        },
        None => { println!("no subcommand :-(") }, // default to list no global?
        _ => {}, // unknown command - show an error.
    }
}
