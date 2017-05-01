use aliases::Config;
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
        let mut config = Config::load();
        match config.users().into_iter().position(|user| {user == self.user.name()}) {
            Some(index) => {
                let mut users = config.users();
                let user = users.remove(index);
                users.insert(self.position - 1, user);
                config.update_users(users);
            },
            None => { println!("Error! Could not find the user {}.", self.user.name()) },
        }
        CommandResponse::success()
    }
}
