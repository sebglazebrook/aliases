extern crate yaml_rust;
extern crate crypto;
extern crate rustc_serialize;

mod aliases;

use aliases::commands::init::Init;
use aliases::commands::list::List;
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
use std::io::prelude::*;
use std::fs::File;
use rustc_serialize::json;
use std::process::Command;

struct TemplateRepository;

impl TemplateRepository {

    pub fn config_template() -> String {
        "{
            \"shim_directory\" : \"${HOME}/.aliases.d/shims\",
            \"alias_directories\" : []
        }
        ".to_string() // TODO what will this actually be?
    }
}

#[derive(Clone, RustcDecodable, RustcEncodable)]
pub struct Config {
    shim_directory: String,
    alias_directories: Vec<String>,
}

impl Config {

    pub fn load() -> Config {
        if Config::config_file_path().exists() {
            Config::load_file(&Config::config_file_path())
        } else {
            Config::create(&Config::config_file_path())
        }
    }

    pub fn shim_path(&self) -> PathBuf {
        let mut command = String::from("echo \"");
        command.push_str(&self.shim_directory);
        command.push_str("\"");
        let output = Command::new("bash")
            .arg("-c")
            .arg(&command)
            .output()
            .unwrap_or_else(|e| { panic!("failed to execute child: {}", e) });

        PathBuf::from(String::from_utf8(output.stdout).unwrap().trim())
    }

    pub fn alias_paths(&self) -> Vec<PathBuf> {
        self.alias_directories.iter().map(|dir| PathBuf::from(dir)).collect()
    }

    pub fn add_alias_directory(&mut self, directory: &PathBuf) {
        let string = String::from(directory.to_str().unwrap());
        self.alias_directories.push(string);
        self.update_file();
    }

    // ------- private methods --------//

    fn config_file_path() -> PathBuf {
        match env::var("HOME") {
            Ok(home_dir) => {
                PathBuf::from(home_dir).join(".aliases_cfg")
            },
            Err(_) => {
                PathBuf::new() // need to handle this better
            },
        }
    }

    fn create(path: &PathBuf) -> Config {
        let mut file = File::create(path).unwrap(); //TODO handle the error case
        let default_config = TemplateRepository::config_template();
        file.write_all(default_config.as_bytes()).unwrap();
        let config: Config = json::decode(&default_config).unwrap();
        config
    }

    fn load_file(path: &PathBuf) -> Config {
        let mut file = File::open(path).unwrap(); //TODO handle the error case
        let mut content = String::new();
        let _ = file.read_to_string(&mut content);
        let config: Config = json::decode(&content).unwrap();
        config
    }

    fn update_file(&self) {
        let mut file = File::create(Config::config_file_path()).unwrap(); //TODO handle the error case
        let encoded = json::encode(&self).unwrap();
        file.write_all(encoded.as_bytes()).unwrap();
    }
}


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

    pub fn execute_exec(&mut self, directory: String, name: String) {
        Exec::new(directory, name).execute();
    }
}
