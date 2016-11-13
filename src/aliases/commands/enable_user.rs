use aliases::Config;

pub struct EnableUser {
    username: String,
}

impl EnableUser {

    pub fn new(username: String) -> Self {
        EnableUser { username: username }
    }

    pub fn execute(&self) {
        let mut config = Config::load();
        config.enable_user(&self.username);
    }

}
