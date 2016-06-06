use aliases::commands::{Init, List, Rehash, Exec, Users};
use aliases::Config;

use std::env;
use std::path::{PathBuf};
use std::process;

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
        let exit_code = List::new(self.current_path.clone(), directory, name, self.config.alias_paths()).execute();
        process::exit(exit_code);
    }

    pub fn execute_rehash(&mut self) {
        // TODO this also needs an exit code
        Rehash::new(self.config.shim_path(), self.config.alias_paths()).execute();
    }

    pub fn execute_exec(&mut self, directory: String, name: String, forwarding_args: Vec<String>) {
        Exec::new(directory, name, forwarding_args).execute();
    }

    pub fn execute_users(&mut self) {
        Users::new(self.config.clone()).execute();
    }
}
