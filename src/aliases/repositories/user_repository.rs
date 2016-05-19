use std::path::PathBuf;

use aliases::models::User;

pub struct UserRepository;

impl UserRepository {

    pub fn all() -> Vec<String> { // TODO this should return Vec<User>
        // TODO get data from config and build users
        vec!["default".to_string()]
    }

    //----------- private -----------//

    fn user_data_filepath() -> PathBuf {
        PathBuf::from("~/.aliases.d/users.yml")
    }

}
