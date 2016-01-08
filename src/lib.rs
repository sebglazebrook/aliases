use std::env;
use std::path::{Path, PathBuf};
use std::io::prelude::*;
use std::fs::File;


pub struct App {
    pub target_path: PathBuf,
}

impl App {

    pub fn new() -> App {
        App { target_path: env::current_dir().unwrap() }
    }

    pub fn execute_init(&mut self, global: bool) {
        if global {
            match env::var("HOME") {
                Ok(home_dir) => {
                    self.target_path = PathBuf::from(home_dir);
                    if Path::new(&self.target_path.join(".aliases")).exists() {
                        println!("Directory already initialized.");
                    } else {
                        let mut template_file = self.template_file();
                        let mut template_contents = String::new();
                        template_file.read_to_string(&mut template_contents).unwrap();
                        let mut new_file = File::create(self.target_path.join(".aliases")).unwrap();
                        let array = template_contents.as_bytes();
                        new_file.write_all(array);
                    }
                },
                Err(e) => {},
            }
        } else {
            if Path::new(&self.target_path.join(".aliases")).exists() {
                println!("Directory already initialized.");
            } else {
                let mut template_file = self.template_file();
                let mut template_contents = String::new();
                template_file.read_to_string(&mut template_contents).unwrap();
                let mut new_file = File::create(self.target_path.join(".aliases")).unwrap();
                let array = template_contents.as_bytes();
                new_file.write_all(array);
            }
        }
    }

    // ------------ private ---------- //

    fn template_file(&self) -> File {
        let template_file_path = Path::new("src/templates/aliases"); // how will I work out this path?
        File::open(template_file_path).unwrap()
    }

    //pub fn execute_list
    //pub fn execute_test
    //pub fn bash autocomplete

}
