use std::env;

use aliases::Config;
use aliases::models::User;

pub struct UserRepository;

impl UserRepository {

    pub fn all() -> Vec<User> {
        Config::load().users().into_iter().fold(Self::trumping_users(), |mut acc, username| {
            let user = User::new(username);
            if !(Self::trumping_users().contains(&user)) {
                acc.push(user);
            }
            acc
        })
    }

    fn trumping_users() -> Vec<User> {
        match env::var("ALIASES_USER") {
            Err(_) => { vec![] },
            Ok(username) => {
                vec![User::new(String::from(username))]
            }
        }
    }

}
