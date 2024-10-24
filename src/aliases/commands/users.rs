use aliases::Config;
use aliases::commands::{AliasCommand, CommandResponse};

pub struct Users {
    config: Config,
}

impl Users {

    pub fn new(config: Config) -> Self {
        Users { config }
    }

}

impl AliasCommand for Users {

    fn execute(&self) -> CommandResponse {
        println!("Users");
        println!("-----");
        println!("{:?}", self.config.users());
        CommandResponse::success()
    }
}
