use aliases::repositories::AliasFileRepository;
use aliases::models::Alias;
use aliases::Config;
use std::path::PathBuf;
use std::io::Write;

macro_rules! println_stderr(
    ($($arg:tt)*) => { {
        let result =  writeln!(&mut ::std::io::stderr(), $($arg)*);
        result.expect("failed printing to stderr");
    } }
);

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
        match alias_file.remove_alias(self.build_alias()) {
            Ok(_) => {
                AliasFileRepository::save(alias_file);
                0
            }
            Err(_) => {
                println_stderr!("Could not find alias \"{}\" in directory {:?}", self.build_alias().name, self.directory);
                1
            }
        }
    }

    //----- private -----//

    fn build_alias(&self) -> Alias {
        let mut alias = Alias::new();
        alias.name = self.name.clone();
        alias
    }

}
