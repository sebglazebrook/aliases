use std::fs;
use std::process::Command;
use std::path::Path;

pub struct Git;

impl Git {

    pub fn clone(url: String, output_dir: &String) -> Result<(), String> {
        Self::create_parent_directories(output_dir);
        match Self::git_clone(url, output_dir) {
            Err(_) => { return Err(String::from("An error occurred")); }, // TODO handle this error case better
            Ok(_) => {
                Ok(())
            }
        }
    }

    fn create_parent_directories(output_dir: &String) {
        let parent_dir = Path::new(output_dir).parent().unwrap();
        fs::create_dir_all(parent_dir); // TODO handle the result of this
    }

    fn git_clone(repo_url: String, output_directory: &str) -> Result<(), &str> {
        let result = Command::new("git")
                             .arg("clone")
                             .arg(repo_url)
                             .arg(output_directory)
                             .output()
                             .expect("failed to clone repo");
        if result.status.success() {
            Ok(())
        } else {
            Err("an error occurred trying to clone the repo") // TODO improve this error message most likely repo doesn't exist
        }
    }

}
