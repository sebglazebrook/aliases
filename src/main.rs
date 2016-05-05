#[macro_use]
extern crate clap;
extern crate aliases;

use aliases::App;

fn main() {
    let yaml = load_yaml!("../config/cli.yml");
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
                let mut forwarding_args: Vec<&str> = matches.values_of("name").unwrap().collect();
                forwarding_args.remove(0);
                let forwarding_args = forwarding_args.into_iter().map(|arg| arg.to_string() ).collect();
                App::new().execute_exec(directory, command_name, forwarding_args);
            }
        },
        Some("users") => {
            if let Some(_) = matches.subcommand_matches("users") {
                App::new().execute_users();
            }
        },
        None => {
            App::new().execute_list(None, None);
        },
        _ => {}, // unknown command - show an error.
    }
}
