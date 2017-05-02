use std::env;

use aliases::Config;
use aliases::commands::{CommandResponse, AliasCommand};
use aliases::repositories::UserRepository;
use aliases::models::User;

pub struct CloneRepo {
    user: User,
    repo_url: Option<String>,
    enable: bool,
}

impl CloneRepo {

    pub fn new(username: String, repo_url: Option<&str>, enable: bool) -> Self {
        CloneRepo { 
            user: UserRepository::find_by_name_or_blow(&username),
            repo_url: repo_url.map(|url| url.to_owned()),
            enable,
        }
    }

    fn enable_user(&self) {
        if self.enable {
            self.user.enable();
        }
    }

}

impl AliasCommand for CloneRepo {

    fn execute(&self) -> CommandResponse {
        match self.user.clone_external_repo(self.repo_url.clone()) {
            Err(message) => CommandResponse::new(1, Some(message)),
            Ok(_) => {
                self.enable_user();
                CommandResponse::success()
            },
        }
    }
}
