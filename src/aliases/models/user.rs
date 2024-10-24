use std::env;
use std::path::PathBuf;
use std::io;
use std::os::unix;

use aliases::repositories::AliasFileRepository;
use aliases::{Config, Git};

#[derive(PartialEq,Eq,Debug,Clone)]
pub struct User {
    name: String,
    filename: String,
    enabled: bool,
}

impl User {

    pub fn new(name: String, enabled: bool) -> Self {
        let filename = match name.as_ref() {
            "default" => String::from(".aliases"),
            _ => format!(".aliases-{}", &name),
        };
        User { filename, name, enabled }
    }

    pub fn filename(&self) -> String {
        self.filename.clone()
    }

    pub fn is_enabled(&self) -> bool {
        self.enabled
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
        AliasFileRepository::create(&target_dir, &self.filename)?;
        Config::load().add_alias_directory(&target_dir, &self.name);
        Ok(())
    }

    pub fn enable(&self) -> Result<(), io::Error> {
        Config::load().enable_user(&self.name)
    }

    pub fn disable(&self) {
        Config::load().disable_user(&self.name);
    }

   pub fn set_priority(&self, position: usize) -> Result<(), String> {
        Config::load().set_user_priority(&self.name, position)
    }

   pub fn clone_external_repo(&self, url: Option<String>) -> Result<(), String> {
       let external_url = url.unwrap_or(self.default_external_url());
       match Git::clone(external_url, &self.output_directory()) {
           Err(message) => Err(message),
           Ok(_) => {
               self.link_aliases_file();
               Ok(())
           }
       }
   }

   fn default_external_url(&self) -> String {
       format!("https://github.com/{}/dot-aliases", self.name)
   }

   fn output_directory(&self) -> String {
       self.output_base_dir() + "/"  + &self.name
   }

   fn output_base_dir(&self) -> String {
       match env::var("HOME") {
           Err(_) => { String::from("No $HOME environment variable set. Don't know your home directory") },
           Ok(home_dir) => {
               format!("{}/.aliases.d/users", home_dir)
           },
       }
   }

    fn link_aliases_file(&self) {
        let target_file = self.output_directory() + "/.aliases";
        let destination_file = env::var("HOME").unwrap().to_string() + &self.filename; // TODO handle this better;
        unix::fs::symlink(target_file, destination_file); // TODO handle the result of this and what about if it's not unix?
    }

}
