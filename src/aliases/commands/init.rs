use std::path::{Path, PathBuf};
use std::io::prelude::*;
use std::fs::File;
use super::super::super::Config; // TODO fix this please!

pub struct Init {
    target_path: PathBuf,
    config: Config,
}

impl Init {

    pub fn new(target_path: PathBuf, config: Config) -> Init {
        Init { target_path: target_path, config: config}
    }

    pub fn execute(&mut self) {
        if Path::new(&self.target_path.join(".aliases")).exists() {
            println!("Directory already initialized.");
        } else {
            let mut template_file = self.template_file();
            let mut template_contents = String::new();
            template_file.read_to_string(&mut template_contents).unwrap();
            let mut new_file = File::create(self.target_path.join(".aliases")).unwrap();
            let array = template_contents.as_bytes();
            let _ = new_file.write_all(array);
            self.add_to_global_config();
        }
    }

    // ------------ private ---------- //

    fn template_file(&self) -> File {
        let template_file_path = Path::new("src/templates/aliases"); // how will I work out this path?
        File::open(template_file_path).unwrap()
    }

    fn add_to_global_config(&mut self) {
        self.config.add_alias_directory(&self.target_path);
    }
}
