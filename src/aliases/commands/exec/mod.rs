use std::path::PathBuf;
use aliases::factories::AliasFactory;

pub struct Exec {
    directory: String,
    name: String,
}

impl Exec {

    pub fn new(directory: String, name: String) -> Self {
        Exec {
            directory: directory,
            name: name,
        }
    }

    pub fn execute(&self) {
        let aliases_file =  PathBuf::from(&self.directory).join(".aliases");
        match AliasFactory::create_from_file(aliases_file) {
            Err(_) => {}, // TODO handle this
            Ok(aliases) => {
                aliases.raw_collection.iter().find(|alias| {
                    alias.name == self.name
                }).unwrap().execute();
           }
        }
    }
}
