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
        match self.user.enable() {
            Ok(_) => CommandResponse::success(),
            Err(error) => CommandResponse::error(1, Some(error.to_string().to_owned())),
        }
    }

}
