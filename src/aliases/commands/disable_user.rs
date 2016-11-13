use aliases::Config;

pub struct DisableUser {
    username: String,
}

impl DisableUser {

    pub fn new(username: String) -> Self {
        DisableUser { username: username }
    }

    pub fn execute(&self) {
        let mut config = Config::load();
        config.disable_user(&self.username);
    }

}
