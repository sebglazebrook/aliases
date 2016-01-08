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
        None => { println!("no subcommand :-(") },
        _ => {},
    }
}
