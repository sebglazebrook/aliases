use aliases::Config;
use aliases::commands::{AliasCommand, CommandResponse};
use aliases::repositories::UserRepository;
use aliases::models::User;

pub struct EnableUser {
    user: User,
}

impl EnableUser {

    pub fn new(username: String) -> Self {
        EnableUser {
            user: UserRepository::find_by_name_or_blow(&username),
        }
    }
}

impl AliasCommand for EnableUser {

    fn execute(&self) -> CommandResponse {
        let mut config = Config::load();
        config.enable_user(&self.user.name());
        CommandResponse::success()
    }

}
