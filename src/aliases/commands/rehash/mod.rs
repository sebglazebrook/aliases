use std::path::PathBuf;
use std::fs;
use std::fs::File;

use aliases::factories::AliasFactory;
use aliases::models::Alias;

pub struct Rehash {
    pub shim_directory: PathBuf,
    pub alias_directories: Vec<PathBuf>,
}

impl Rehash {

    pub fn new() -> Rehash {
        Rehash { 
            shim_directory: PathBuf::new(),
            alias_directories: vec![],
        }
    }

    pub fn execute(&self) {
        for dir in &self.alias_directories {
            match AliasFactory::create_from_file(dir.join(".aliases")) {
                Err(_) => {}, // TODO
                Ok(aliases) => {
                    for alias in aliases.raw_collection.iter() { // i know don't touch the raw collection have to fix this.
                        self.create_global_shim_file(&alias);
                        self.create_specific_shim_file(&alias, &dir);
                    }
                }
            }
        }
    }

    //------- private methods --------//
    
    fn create_global_shim_file(&self, alias: &Alias) {
        let filepath = self.shim_directory.join(alias.name.clone());
        if !filepath.exists() {
            File::create(filepath); // TODO don't just create a file, create it with content
        }
    }

    fn create_specific_shim_file(&self, alias: &Alias, alias_directory: &PathBuf) {
        let nested_path = alias_directory.join(alias.name.clone());
        let shim_specific_path;
        if nested_path.has_root() {
            let mut string = String::from(nested_path.to_str().unwrap()); // TODO handle the none option??
            string.remove(0);
            shim_specific_path = self.shim_directory.join(string);
        } else {
            shim_specific_path = self.shim_directory.join(nested_path);
        }
        if !shim_specific_path.exists() {
            fs::create_dir_all(shim_specific_path.parent().unwrap()); // TODO handle the none case??
            File::create(shim_specific_path); // TODO don't just create a file, create it with content
        }
    }
}
