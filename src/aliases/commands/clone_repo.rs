use std::fs;
use std::env;
use std::process::Command;
use std::os::unix;

use aliases::Config;
use aliases::commands::{CommandResponse, AliasCommand};
use aliases::repositories::UserRepository;
use aliases::models::User;

pub struct CloneRepo<'a> {
    user: User,
    repo_url: Option<&'a str>,
    enable: bool,
}

impl<'a> CloneRepo<'a> {

    pub fn new(username: String, repo_url: Option<&'a str>, enable: bool) -> Self {
        CloneRepo { 
            user: UserRepository::find_by_name_or_blow(&username),
            repo_url,
            enable,
        }
    }

    fn output_base_dir(&self) -> String {
        match env::var("HOME") {
            Err(_) => { String::from("No $HOME environment variable set. Don't know your home directory") },
            Ok(home_dir) => {
                format!("{}/.aliases.d/users", home_dir) 
            },
        }
    }

    fn link_aliases_file(&self) {
        let target_file = self.output_directory() + "/.aliases";
        let destination_file = env::var("HOME").unwrap().to_string() + &self.user.filename(); // TODO handle this better;
        unix::fs::symlink(target_file, destination_file); // TODO handle the result of this and what about if it's not unix?
    }
    
    fn prepare_output_dir(&self) {
        fs::create_dir_all(&self.output_base_dir()); // TODO handle the result of this
    }

    fn git_clone(&self, repo_url: &str, output_directory: &str) -> Result<(),&str> {
        let result = Command::new("git")
                             .arg("clone")
                             .arg(repo_url)
                             .arg(output_directory)
                             .output()
                             .expect("failed to clone repo");
        if result.status.success() {
            Ok(())
        } else {
            Err("an error occurred trying to clone the repo") // TODO improve this error message most likely repo doesn't exist
        }
    }

    fn repo_url(&self) -> String {
        match self.repo_url {
            Some(url) => { url.to_string() },
            None => { format!("https://github.com/{}/dot-aliases", self.user.name()) },
        }
    }

    fn output_directory(&self) -> String {
        self.output_base_dir() + "/"  + &self.user.name()
    }

    fn enable_user(&self) {
        if self.enable {
            Config::load().enable_user(&self.user.name());
        }
    }

}

impl<'a> AliasCommand for CloneRepo<'a> {

    fn execute(&self) -> CommandResponse {
        self.prepare_output_dir();
        match self.git_clone(&self.repo_url(), &self.output_directory()) {
            Err(_) => { return CommandResponse::new(1, Some(String::from("An error occurred"))); }, // TODO handle this error case better
            Ok(_) => {
                self.link_aliases_file();
                self.enable_user();
            }
        }
        CommandResponse::success()
    }
}
