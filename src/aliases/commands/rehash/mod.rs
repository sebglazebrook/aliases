use std::path::PathBuf;

use aliases::factories::{AliasFactory, ShimFileFactory};

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
                        ShimFileFactory::create_global(&alias, &self.shim_directory);
                        ShimFileFactory::create_specific(&alias, &dir, &self.shim_directory);

                    }
                }
            }
        }
    }
}
