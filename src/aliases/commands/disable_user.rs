use aliases::Config;
use aliases::commands::{CommandResponse, AliasCommand};
use aliases::repositories::UserRepository;
use aliases::models::User;

pub struct DisableUser {
    user: User,
}

impl DisableUser {

    pub fn new(username: String) -> Self {
        DisableUser {
            user: UserRepository::find_by_name_or_blow(&username),
        }
    }

}

impl AliasCommand for DisableUser {

    fn execute(&self) -> CommandResponse {
        self.user.disable();
        CommandResponse::success()
    }
}
