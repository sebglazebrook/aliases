use std::env;
use std::process::Command;

pub struct PullRepo<'a> {
    username: Option<&'a str>,
}

impl<'a> PullRepo<'a> {

    pub fn new(username: Option<&'a str>) -> Self {
        PullRepo { username: username }
    }

    pub fn execute(&self) {
        let result = Command::new("git")
                             .arg("pull")
                             .current_dir(self.repo_dir())
                             .output()
                             .expect("failed to pull repo");
        if result.status.success() {
            //Ok(())
        } else {
            //Err("an error occurred trying to pull the repo") // TODO improve this error message
        }
    }

    fn repo_dir(&self) -> String {
        match self.username {
            None => { String::new() }, // TODO handle this
            Some(username) => {
                match env::var("HOME") {
                    Err(_) => { String::new() },  // TODO handle this
                    Ok(home_dir) => {
                        format!("{}/.aliases.d/users/{}", home_dir, username) 
                    },
                }
            }
        }
    }

}
