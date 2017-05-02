use aliases::commands::{CommandResponse, AliasCommand};
use aliases::repositories::UserRepository;
use aliases::models::User;

pub struct MoveUser {
    user: User,
    position: usize,
}

impl MoveUser {

    pub fn new(username: String, position: usize) -> Self {
        MoveUser {
            user: UserRepository::find_by_name_or_blow(&username),
            position,
        }
    }

}

impl AliasCommand for MoveUser {

    fn execute(&self) -> CommandResponse {
        match self.user.set_priority(self.position) {
            Ok(_) => { CommandResponse::success() },
            Err(message) => {
                println!("{}", message);
                CommandResponse::new(1, Some(message))
            },
        }
    }
}
