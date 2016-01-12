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
                App::new().execute_list();
            }
        },
        //Some("rehash") => { // updates the aliases
            //if let Some(matches) = matches.subcommand_matches("rehash") {
                //App::new().execute_rehash();
            //}
        //},
        //Some("add") => { // interactive console to add an alias
            //if let Some(matches) = matches.subcommand_matches("add") {
                //App::new().execute_add();
            //}
        //},
        //Some("remove") => { // remove an aliases
            //if let Some(matches) = matches.subcommand_matches("remove") {
                //App::new().execute_remove();
            //}
        //},
        //Some("test") => { // run the test for one or more aliases
            //if let Some(matches) = matches.subcommand_matches("test") {
                //App::new().execute_test();
            //}
        //},
        None => { println!("no subcommand :-(") }, // default to list no global?
        _ => {}, // unknown command - show an error.
    }
}
