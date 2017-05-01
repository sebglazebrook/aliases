use std::env;
use std::path::PathBuf;
use std::io;

use aliases::repositories::AliasFileRepository;
use aliases::Config;

#[derive(PartialEq,Eq,Debug,Clone)]
pub struct User {
    name: String,
    filename: String,
    enabled: bool,
}

impl User {

    pub fn new(name: String, enabled: bool) -> Self {
        let filename;
        if name == "default" {
            filename = String::from(".aliases");
        } else {
            filename = format!(".aliases-{}", &name)
        }
        User{filename: filename , name: name, enabled: enabled }
    }

    pub fn filename(&self) -> String {
        self.filename.clone()
    }

    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn confirm_name(&self, other_name: &str) -> bool {
        self.name == other_name
    }

    pub fn home_dir(&self) -> Result<String, &'static str> {
        match env::var("HOME") {
            Err(_) => { Err("Error! Could not evaluate env var $HOME. Can't continue.") },
            Ok(home_dir) => {
                Ok(format!("{}/.aliases.d/users/{}", home_dir, self.name))
            },
        }
    }

    pub fn init_directory(&self, target_dir: &PathBuf) -> Result<(), io::Error> {
        try!(AliasFileRepository::create(&target_dir, &self.filename));
        Config::load().add_alias_directory(&target_dir, &self.name);
        Ok(())
    }

}
