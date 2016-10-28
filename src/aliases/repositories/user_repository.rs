use aliases::Config;
use aliases::models::User;

pub struct UserRepository;

impl UserRepository {

    pub fn all() -> Vec<User> {
        Config::load().users().into_iter().map(|username| {
            User::new(username)
        }).collect()
    }

}
