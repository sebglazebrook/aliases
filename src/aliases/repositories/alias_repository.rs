use std::path::PathBuf;
use aliases::factories::AliasFactory;
use aliases::collections::Aliases;
use aliases::repositories::UserRepository;

pub struct AliasRepository;

impl AliasRepository {

    pub fn find_for_directory(directory: &String) -> Result<Aliases, &'static str> {
        let mut aliases = Aliases::new(vec![]); // TODO should be able to use map or inject here
        for user in Self::all_users() {
            let found_aliases = Self::directory_aliases_for_user(directory, &user).unwrap(); // TODO stop unwrapping
            aliases = aliases.merge(found_aliases);
        }
        Ok(aliases)
    }

    //--------- private ------------//

    fn all_users() -> Vec<String> {
        UserRepository::all()
    }

    fn directory_aliases_for_user(directory: &String, user: &String) -> Option<Aliases> {
        let aliases_file = PathBuf::from(directory).join(".aliases"); // TODO use the user's file name
        match AliasFactory::create_from_file(aliases_file) {
            Err(_) => { None },
            Ok(aliases) => { Some(aliases) }
        }
    }


}
