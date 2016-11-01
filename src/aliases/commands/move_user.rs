use aliases::Config;

pub struct MoveUser {
    username: String,
    position: usize,
}

impl MoveUser {

    pub fn new(username: String, position: usize) -> Self {
        MoveUser { username: username, position: position }
    }

    pub fn execute(&self) {
        let mut config = Config::load();
        match config.users().into_iter().position(|user| {user == self.username}) {
            Some(index) => {
                let mut users = config.users();
                let user = users.remove(index);
                users.insert(self.position - 1, user);
                config.update_users(users);
            },
            None => { println!("Error! Could not find the user {}.", self.username) },
        }

    }

}
