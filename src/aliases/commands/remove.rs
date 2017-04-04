use aliases::repositories::AliasFileRepository;
use aliases::models::Alias;
use aliases::Config;
use std::path::PathBuf;

pub struct Remove {
    directory: PathBuf,
    name: String
}

impl Remove {

    pub fn new(directory: PathBuf, name: Option<&str>) -> Self {
        Remove {
            directory: directory,
            name: name.unwrap().to_string()
        }
    }

    pub fn execute(&self) -> i32 {
        let mut alias_file = AliasFileRepository::find(&self.directory);
        alias_file.remove_alias(self.build_alias());
        AliasFileRepository::save(alias_file);
        0 // TODO make this a real exit code
    }

    //----- private -----//

    fn build_alias(&self) -> Alias {
        let mut alias = Alias::new();
        alias.name = self.name.clone();
        alias
    }

}
