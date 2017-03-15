use aliases::commands::{Init, List, Add, Rehash, Exec, Users, MoveUser, CloneRepo, PullRepo, EnableUser, DisableUser, AliasCommand, Directories};
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

    pub fn execute_init(&mut self, global: bool, user: Option<&str>) {
        Init::new( calculate_target_path_for_init(global, &self.current_path), self.config.clone(), global, user).execute();
    }

    pub fn execute_list(&mut self, directory: Option<&str>, name: Option<&str>) {
        let exit_code = List::new(self.current_path.clone(), directory, name).execute();
        process::exit(exit_code);
    }

    pub fn execute_add(&mut self, name: Option<&str>, command: Option<&str>) {
        let exit_code = Add::new(self.current_path.clone(), name, command).execute();
        process::exit(exit_code);
    }

    pub fn execute_directories(&mut self) {
         let exit_code = Directories::new().execute();
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

    pub fn execute_clone(&mut self, username: String, repo_url: Option<&str>, enable: bool) {
        CloneRepo::new(username, repo_url, enable).execute();
    }

    pub fn execute_pull(&mut self, username: Option<&str>) {
       let response = PullRepo::new(username).execute();
       if response.is_error() {
           response.print_error_message();
       }
    }

    pub fn prioritize_user(&mut self, username: String, position: usize) {
        MoveUser::new(username, position).execute();
    }

    pub fn enable_user(&mut self, username: String) {
        EnableUser::new(username).execute();
    }

    pub fn disable_user(&mut self, username: String) {
        DisableUser::new(username).execute();
    }
}

fn calculate_target_path_for_init(global: bool, current_path: &PathBuf) -> PathBuf {
    match global {
        false => { current_path.clone() }
        true => {
            match env::var("HOME") {
                Ok(home_dir) => {
                    PathBuf::from(home_dir)
                },
                Err(_) => {
                    current_path.clone()
                },
            }
        }
    }
}
