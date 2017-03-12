#[macro_use]
extern crate clap;
extern crate aliases;
extern crate env_logger;

use aliases::App;

fn main() {
    env_logger::init().unwrap();
    let yaml = load_yaml!("../config/cli.yml");
    let matches = clap::App::from_yaml(yaml).get_matches();

    match matches.subcommand_name() {
        Some("init") => {
            if let Some(matches) = matches.subcommand_matches("init") {
                App::new().execute_init(matches.is_present("global"), matches.value_of("user"));
            }
        },
        Some("add") => {
            if let Some(matches) = matches.subcommand_matches("add") {
                App::new().execute_add(matches.value_of("name"), matches.value_of("command"));
            }
        },
        Some("directories") => {
            App::new().execute_directories();
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
            if let Some(subcommand_matches) = matches.subcommand_matches("users") {
                match subcommand_matches.subcommand_name() {
                    Some("move") => {
                        if let Some(move_matches) = subcommand_matches.subcommand_matches("move") {
                        let username = move_matches.value_of("username").unwrap().to_string();
                        // TODO  handle when this isn't an Integer
                        let prioritization = move_matches.value_of("prioritization").unwrap().parse::<usize>().unwrap();
                        App::new().prioritize_user(username, prioritization);
                        }
                    },
                    Some("use") => {
                        if let Some(move_matches) = subcommand_matches.subcommand_matches("use") {
                        let username = move_matches.value_of("username").unwrap().to_string();
                        App::new().prioritize_user(username, 1);
                        }
                    },
                    Some("enable") => {
                        if let Some(move_matches) = subcommand_matches.subcommand_matches("enable") {
                        let username = move_matches.value_of("username").unwrap().to_string();
                        App::new().enable_user(username);
                        }
                    },
                    Some("disable") => {
                        if let Some(move_matches) = subcommand_matches.subcommand_matches("disable") {
                        let username = move_matches.value_of("username").unwrap().to_string();
                        App::new().disable_user(username);
                        }
                    },
                    None => {
                        App::new().execute_users();
                    },
                    _ => {},
                }

            }
        },
        Some("clone") => {
            if let Some(matches) = matches.subcommand_matches("clone") {
                App::new().execute_clone(
                    matches.value_of("username").unwrap().to_string(),
                    matches.value_of("repo_url"),
                    matches.is_present("enable")
                    );
            }
        },
        Some("pull") => {
            if let Some(matches) = matches.subcommand_matches("pull") {
                App::new().execute_pull(matches.value_of("username"));
            }
        },
        None => {
            App::new().execute_list(None, None);
        },
        _ => {}, // unknown command - show an error.
    }
}
