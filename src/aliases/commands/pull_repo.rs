use std::process::Command;

use aliases::commands::{AliasCommand, CommandResponse};
use aliases::models::User;
use aliases::repositories::UserRepository;

pub struct PullRepo {
    user: Option<User>,
}

impl PullRepo {

    pub fn new(username: Option<&str>) -> Self {
        let user = username.map(|name| UserRepository::find_by_name_or_blow(name));
        PullRepo { user: user }
    }

    fn repo_dir(&self) -> Result<String, &str> {
        match self.user {
            None => { Ok(String::new()) }, // TODO this should pull all users
            Some(ref user) => { user.home_dir() }
        }
    }

}

impl AliasCommand for PullRepo {

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
