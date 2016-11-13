use std::env;
use std::process::Command;

use aliases::commands::{AliasCommand, CommandResponse};

pub struct PullRepo<'a> {
    username: Option<&'a str>,
}

impl<'a> PullRepo<'a> {

    pub fn new(username: Option<&'a str>) -> Self {
        PullRepo { username: username }
    }

    fn repo_dir(&self) -> Result<String, &str> {
        match self.username {
            None => { Ok(String::new()) }, // TODO this should pull all users
            Some(username) => {
                match env::var("HOME") {
                    Err(_) => { Err("Error! Could not evaluate env var $HOME. Can't continue.") },
                    Ok(home_dir) => {
                        Ok(format!("{}/.aliases.d/users/{}", home_dir, username))
                    },
                }
            }
        }
    }

}

impl<'a> AliasCommand for PullRepo<'a> {

    fn execute(&self) -> CommandResponse {
        match self.repo_dir() {
            Err(error_message) => {
                CommandResponse::new(1, Some(error_message.to_string()))
            },
            Ok(repo_dir) => {
                let result = Command::new("git")
                    .arg("pull")
                    .current_dir(repo_dir)
                    .output()
                    .expect("Error! Failed to pull repo");
                if result.status.success() {
                    CommandResponse::success()
                } else {
                    CommandResponse::new(1, Some(String::from("an error occurred trying to pull the repo")))
                }
            }
        }
    }
}
