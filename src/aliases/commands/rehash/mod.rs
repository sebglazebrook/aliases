use std::path::PathBuf;
use std::fs;
use scoped_pool::Pool;

use aliases::factories::ShimFileFactory;
use aliases::repositories::AliasRepository;
use aliases::commands::{AliasCommand, CommandResponse};

pub struct Rehash {
    pub shim_directory: PathBuf,
    pub alias_directories: Vec<PathBuf>,
}

impl Rehash {

    pub fn new(shim_directory: PathBuf, alias_directories: Vec<PathBuf>) -> Rehash {
        Rehash { shim_directory, alias_directories }
    }


    //--------  private ----------//

    fn clean_shims_directory(&self) {
        let _ = fs::remove_dir_all(&self.shim_directory);
        let _ = fs::create_dir_all(&self.shim_directory);
    }
}

impl AliasCommand for Rehash {

    fn execute(&self) -> CommandResponse {
        self.clean_shims_directory();
        let pool = Pool::new(4);
        for dir in &self.alias_directories {
            match AliasRepository::find_for_directory(&dir.to_str().unwrap().to_string()) {
                Err(_) => {},
                Ok(aliases) => {
                    pool.scoped(|scope| {
                        for alias in aliases {
                            scope.execute(move || {
                                ShimFileFactory::create(&alias, &self.shim_directory);
                            });
                        }
                    });
                },
            }
        }
        CommandResponse::success()
    }
}
