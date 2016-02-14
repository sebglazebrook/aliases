extern crate yaml_rust;
extern crate crypto;
extern crate rustc_serialize;
extern crate tabwriter;
extern crate countdown;

mod aliases;

use aliases::commands::init::Init;
use aliases::commands::list::List;
use aliases::Config;
pub use aliases::commands::rehash::Rehash;
pub use aliases::commands::exec::Exec;
pub use aliases::builders::AliasBuilder; // had to do this for the tests, why?
pub use aliases::models::Alias; // had to do this for the tests, why?
pub use aliases::models::Conditional; // had to do this for the tests, why?
pub use aliases::factories::AliasFactory; // had to do this for the tests, why?
pub use aliases::collections::Aliases; // had to do this for the tests, why?
pub use aliases::factories::ShimFileFactory; // had to do this for the tests, why?

use std::env;
use std::path::{PathBuf};

pub struct App {
    config: Config,
    pub current_path: PathBuf,
}

impl App {

    pub fn new() -> App {
        let config = Config::load();
        App {
            config: config,
            current_path: env::current_dir().unwrap(),
        }
    }

    pub fn execute_init(&mut self, global: bool) {
        let target_path;
        if global {
            match env::var("HOME") {
                Ok(home_dir) => {
                   target_path = PathBuf::from(home_dir);
                },
                Err(_) => {
                    target_path = self.current_path.clone();
                },
            }
        } else {
            target_path = self.current_path.clone();
        }
        Init::new(target_path, self.config.clone(), global).execute();
    }

    pub fn execute_list(&mut self, directory: Option<&str>, name: Option<&str>) {
        let exit_code = List::new(self.current_path.clone(), directory, name).execute();
        std::process::exit(exit_code);
    }

    pub fn execute_rehash(&mut self) {
        // TODO this also needs an exit code
        Rehash::new(self.config.shim_path(), self.config.alias_paths()).execute();
    }

    pub fn execute_exec(&mut self, directory: String, name: String, forwarding_args: Vec<String>) {
        Exec::new(directory, name, forwarding_args).execute();
    }
}
