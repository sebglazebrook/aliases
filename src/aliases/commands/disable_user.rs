use aliases::Config;
use aliases::commands::{CommandResponse, AliasCommand};

pub struct DisableUser {
    username: String,
}

impl DisableUser {

    pub fn new(username: String) -> Self {
        DisableUser { username: username }
    }

}

impl AliasCommand for DisableUser {

    fn execute(&self) -> CommandResponse {
        let mut config = Config::load();
        config.disable_user(&self.username);
        CommandResponse::success()
    }
}
