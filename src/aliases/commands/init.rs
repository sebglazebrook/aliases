use std::path::PathBuf;

use aliases::Config;
use aliases::commands::{AliasCommand, CommandResponse};
use aliases::models::User;
use aliases::repositories::UserRepository;

pub struct Init {
    target_path: PathBuf,
    config: Config,
    global: bool,
    user: User,
}

impl Init {

    pub fn new(target_path: PathBuf, config: Config, global: bool, username: Option<&str>) -> Init {
        Init {
            target_path,
            config,
            global,
            user: username.map_or(UserRepository::default(), |name| UserRepository::find_by_name_or_create(name) ),
        }
    }

}

impl AliasCommand for Init {

    fn execute(&self) -> CommandResponse {
        if self.global {
            let path_update = String::from("export PATH=\"") + &self.config.shim_directory +  ":${PATH}\"";
            // what the hell was I trying to do here?
            println!("{}\naliases rehash", path_update);
        } else {
            self.user.init_directory(&self.target_path);
        }
        CommandResponse::success()
    }
}
