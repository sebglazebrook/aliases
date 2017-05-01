use std::env;

use aliases::Config;
use aliases::models::User;

pub struct UserRepository;

impl UserRepository {

    pub fn default() -> User {
        Self::all().iter().find(|user| user.confirm_name("default") ).map(|user| user.to_owned()).unwrap()
    }

    pub fn find_by_name_or_blow(username: &str) -> User {
        Self::find_by_name(username).expect("Error! Could not find the user")
    }

    pub fn find_by_name(username: &str) -> Option<User> {
        Self::all().iter().find(|user| user.confirm_name(username) ).map(|user| user.to_owned())
    }

    pub fn find_by_name_or_create(username: &str) -> User {
        match Self::find_by_name(&username) {
            Some(user) => { user }
            None => { User::new(username.to_string(), true) }
        }
    }

    pub fn enabled() -> Vec<User> {
        Self::all().iter()
            .filter(|user| user.is_enabled())
            .cloned()
            .collect()
    }

    pub fn all() -> Vec<User> {
        Config::load().users().into_iter().fold(Self::trumping_users(), |mut acc, username| {
            let enabled = Self::is_user_enabled(&username);
            let user = User::new(username, enabled);
            if !(Self::trumping_users().contains(&user)) {
                acc.push(user);
            }
            acc
        })
    }


    //------------- private -----------//

    fn trumping_users() -> Vec<User> {
        match env::var("ALIASES_USER") {
            Err(_) => { vec![] },
            Ok(username) => {
                vec![User::new(String::from(username), true)]
            }
        }
    }

    fn is_user_enabled(username: &str) -> bool {
        let disabled_users = Config::load().disabled_users();
        !disabled_users.iter().any(|&ref u| u == username)

    }
}
