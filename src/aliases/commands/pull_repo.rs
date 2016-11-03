use std::env;
use std::process::Command;

pub struct PullRepo<'a> {
    username: Option<&'a str>,
}

impl<'a> PullRepo<'a> {

    pub fn new(username: Option<&'a str>) -> Self {
        PullRepo { username: username }
    }

    pub fn execute(&self) -> Result<(), &str> {
        let result = Command::new("git")
                             .arg("pull")
                             .current_dir(try!(self.repo_dir()))
                             .output()
                             .expect("Error! Failed to pull repo");
        if result.status.success() {
            Ok(())
        } else {
            Err("an error occurred trying to pull the repo") // TODO improve this error message
        }
    }

    fn repo_dir(&self) -> Result<String, &str> {
        match self.username {
            None => { Ok(String::new()) }, // TODO this should pull all users
            Some(username) => {
                match env::var("HOME") {
                    Err(_) => { Err("Error! Could not evaluate env var $HOME. Can't continue.") },
                    Ok(home_dir) => {
                        Ok(format!("{}/.aliases.d/users/{}", home_dir, username))
                    },
                }
            }
        }
    }

}
