use aliases::repositories::AliasFileRepository;
use aliases::models::Alias;
use aliases::Config;
use std::path::PathBuf;
use aliases::commands::{Init, Rehash, AliasCommand};

pub struct Add {
    directory: PathBuf,
    name: String,
    command: String,
}

impl Add {

    pub fn new(directory: PathBuf, name: Option<&str>, command: Option<&str>) -> Self {
        Add {
            directory: directory,
            name: name.unwrap().to_string(), // TODO
            command: command.unwrap().to_string() // TODO
        }
    }

    pub fn execute(&self) -> i32 {
        let mut alias_file = AliasFileRepository::find(&self.directory);
        alias_file.add_alias(self.build_alias());
        AliasFileRepository::save(alias_file);
        let config = Config::load();
        Init::new(self.directory.clone(), config.clone(), false, None).execute();
        Rehash::new(config.shim_path(), config.alias_paths()).execute();
        0 // TODO make this a real exit code
    }

    //----- private -----//

    fn build_alias(&self) -> Alias {
        let mut alias = Alias::new();
        alias.name = self.name.clone();
        alias.command = self.command.clone();
        alias
    }

}
