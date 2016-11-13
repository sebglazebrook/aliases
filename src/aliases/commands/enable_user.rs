use aliases::Config;
use aliases::commands::{AliasCommand, CommandResponse};

pub struct EnableUser {
    username: String,
}

impl EnableUser {

    pub fn new(username: String) -> Self {
        EnableUser { username: username }
    }
}

impl AliasCommand for EnableUser {

    fn execute(&self) -> CommandResponse {
        let mut config = Config::load();
        config.enable_user(&self.username);
        CommandResponse::success()
    }

}
